// The replay: plays the recorded events of a finished game on the map, one every half second, with every
// token in sight (the professor, his hidden counters and Nyog'Sothep's city included).
// #replay-data holds the French names and the steps; a step's board is only present when it changed.
(function () {
  const source = document.getElementById('replay-data');
  const board = document.querySelector('.board');
  if (!source || !board) return;
  const data = JSON.parse(source.textContent);
  // The app is served under a sub-path: images are found next to this script's folder.
  const IMG = new URL('../img/', document.currentScript.src).href;
  const DELAY = 500;
  // Same spacing as the game page (board_view.rs).
  const STACK_STEP = 46;
  const steps = data.steps;
  const boards = [];
  steps.forEach(function (step, index) {
    boards.push(step.board || boards[index - 1]);
  });

  const field = function (name) { return document.querySelector('[data-replay="' + name + '"]'); };
  const progress = field('progress');
  const log = field('log');
  const playButton = document.querySelector('[data-replay-action="play"]');
  const drawn = new Map();
  let current = 0;
  let timer = null;

  function element(tag, attributes, children) {
    const node = document.createElement(tag);
    Object.keys(attributes || {}).forEach(function (name) { node.setAttribute(name, attributes[name]); });
    (children || []).forEach(function (child) {
      node.append(child);
    });
    return node;
  }

  function image(src, title, className) {
    return element('img', { src: src, alt: title, title: title, class: className || '' });
  }

  function place(code) { return data.places[code]; }

  // A slight tilt that stays the same while the token stays on its location, like on the game page.
  function tilt(key, at) {
    let hash = 0;
    const text = key + '@' + at;
    for (let index = 0; index < text.length; index++) hash = (hash * 31 + text.charCodeAt(index)) | 0;
    return ((Math.abs(hash) % 101) - 50) / 10;
  }

  // The tokens of a board, location by location, in the drawing order of the game page.
  function tokensOf(state) {
    const byPlace = new Map();
    function add(at, token) {
      if (!byPlace.has(at)) byPlace.set(at, []);
      byPlace.get(at).push(token);
    }
    state.destroyed.forEach(function (city) {
      add(city, { key: 'destroyed:' + city, image: IMG + 'token-destroyed-city.png', title: place(city).name + ' est détruite' });
    });
    state.counters.forEach(function (counter) {
      const name = data.counters[counter.kind];
      add(counter.city, {
        key: 'counter:' + counter.city + ':' + counter.kind,
        image: IMG + 'token-' + counter.kind + '.png',
        title: name + ' à ' + place(counter.city).name + (counter.hidden ? ' (caché)' : ''),
        className: counter.hidden ? 'hidden' : '',
      });
    });
    if (state.nyog) {
      add(state.nyog, { key: 'nyog', image: IMG + 'token-nyog-sothep.png', title: "Nyog'Sothep " + place(state.nyog).at });
    } else if (state.nyog_city) {
      add(state.nyog_city, {
        key: 'nyog',
        image: IMG + 'token-nyog-sothep.png',
        title: "Nyog'Sothep apparaîtra " + place(state.nyog_city).at,
        className: 'hidden',
      });
    }
    if (state.professor) {
      add(state.professor, {
        key: 'professor',
        image: IMG + 'token-professor.png',
        title: 'Le professeur Petersen ' + place(state.professor).at,
        className: 'professor',
      });
    }
    state.investigators.forEach(function (investigator) {
      if (investigator.status !== 'Active') return;
      const names = data.investigators[investigator.id];
      add(investigator.location, {
        key: 'investigator:' + investigator.id,
        image: IMG + 'token-investigator-' + investigator.id + '.png',
        title: names.name + ' — ' + investigator.san + ' SAN',
        className: 'investigator',
        atSea: place(investigator.location).sea,
      });
    });
    const tokens = [];
    byPlace.forEach(function (stack, at) {
      const info = place(at);
      stack.forEach(function (token, index) {
        token.at = at;
        token.x = info.x + ((2 * index - (stack.length - 1)) * STACK_STEP) / 2;
        token.y = info.y;
        tokens.push(token);
      });
    });
    return tokens;
  }

  // Tokens keep their element from one step to the next, so that a move slides them across the map.
  function drawTokens(state) {
    const kept = new Set();
    tokensOf(state).forEach(function (token) {
      kept.add(token.key);
      let node = drawn.get(token.key);
      if (!node) {
        node = element('span', { class: 'token' }, [image(token.image, token.title)]);
        board.appendChild(node);
        drawn.set(token.key, node);
      }
      node.className = 'token ' + (token.className || '');
      node.title = token.title;
      node.style.left = token.x + 'px';
      node.style.top = token.y + 'px';
      node.style.setProperty('--tilt', tilt(token.key, token.at) + 'deg');
      const boat = node.querySelector('.boat');
      if (token.atSea && !boat) node.appendChild(image(IMG + 'boat.png', 'en mer', 'boat'));
      if (!token.atSea && boat) boat.remove();
    });
    drawn.forEach(function (node, key) {
      if (kept.has(key)) return;
      node.remove();
      drawn.delete(key);
    });
  }

  function investigatorChip(investigator) {
    const names = data.investigators[investigator.id];
    const body = element('span', { class: 'chip-body' }, [element('span', { class: 'chip-name' }, [names.name])]);
    if (investigator.status === 'Active') {
      body.append(element('span', { class: 'chip-stats' }, [investigator.san + ' SAN · ' + place(investigator.location).at]));
      const items = element('span', { class: 'chip-items' });
      if (investigator.weapon) items.append(image(IMG + 'token-weapon.png', 'Arme'));
      if (investigator.sign) items.append(image(IMG + 'token-sign.png', 'Signe protecteur'));
      if (investigator.medallion) items.append(image(IMG + 'token-medallion.png', 'Médaillon de Fanatique'));
      if (investigator.spell) items.append(element('span', { class: 'spell' }, ['✶ sort de renvoi']));
      body.append(items);
    } else {
      const fate = investigator.status === 'Mad' ? 'devenu' + names.agreement + ' Fanatique' : 'mort' + names.agreement;
      body.append(element('span', { class: 'chip-note' }, [fate]));
    }
    return element('li', { class: 'investigator-chip status-' + investigator.status.toLowerCase() }, [
      image(IMG + 'token-investigator-' + investigator.id + '.png', '', 'chip-token'),
      body,
    ]);
  }

  function drawBanner(state) {
    const hand = element('p', { class: 'banner-hand' }, ['Main du professeur :']);
    state.hand.forEach(function (kind) {
      hand.append(image(IMG + 'token-' + kind + '.png', data.counters[kind], 'resource-icon'));
    });
    if (state.hand.length === 0) hand.append(element('span', {}, ['vide']));
    let nyog = "Nyog'Sothep : ville d'apparition pas encore choisie.";
    if (state.nyog) nyog = "Nyog'Sothep est " + place(state.nyog).at + '.';
    else if (state.nyog_city) nyog = "Nyog'Sothep apparaîtra " + place(state.nyog_city).at + '.';
    const professor = state.professor ? place(state.professor).at : 'point de départ pas encore choisi';
    field('banner').replaceChildren(
      element('p', { class: 'banner-professor' }, [
        image(IMG + 'token-professor.png', '', 'resource-icon'),
        element('span', {}, [
          element('strong', {}, ['Professeur Petersen']),
          ' : ' + state.professor_hp + ' points de vie, ' + professor,
        ]),
      ]),
      hand,
      element('p', { class: 'banner-professor' }, [image(IMG + 'token-nyog-sothep.png', '', 'resource-icon'), nyog]),
      element('ul', { class: 'investigators' }, state.investigators.map(investigatorChip))
    );
  }

  function turnLabel(turn) {
    return turn === 0 ? 'Mise en place' : 'Tour ' + turn;
  }

  function stepText(step) {
    if (step.text) return step.text;
    return step.dice.length > 0 ? 'Jet de dés.' : 'Le plateau change.';
  }

  function diceText(dice) {
    return dice.map(function (value) { return String.fromCodePoint(0x2680 + value - 1); }).join(' ');
  }

  // The journal lists every step, newest first; the steps still to come stay hidden.
  function buildLog() {
    steps.forEach(function (step, index) {
      const item = element('li', { 'data-step': index }, [
        element('span', { class: 'log-turn' }, ['T' + step.turn]),
        ' ' + stepText(step),
      ]);
      if (step.dice.length > 0) item.append(element('span', { class: 'log-dice' }, [' ' + diceText(step.dice)]));
      item.addEventListener('click', function () {
        pause();
        show(index);
      });
      log.prepend(item);
    });
  }

  function show(index) {
    current = Math.max(0, Math.min(index, steps.length - 1));
    const step = steps[current];
    drawTokens(boards[current]);
    drawBanner(boards[current]);
    field('turn').textContent = turnLabel(step.turn);
    field('position').textContent = 'événement ' + (current + 1) + ' sur ' + steps.length;
    field('text').textContent = stepText(step);
    field('dice').textContent = step.dice.length > 0 ? 'Dés : ' + diceText(step.dice) : '';
    field('outcome').hidden = current !== steps.length - 1;
    progress.value = current;
    log.querySelectorAll('li').forEach(function (item) {
      const position = Number(item.dataset.step);
      item.hidden = position > current;
      item.classList.toggle('is-current', position === current);
    });
  }

  function pause() {
    clearTimeout(timer);
    timer = null;
    playButton.textContent = '▶';
  }

  function tick() {
    if (current >= steps.length - 1) return pause();
    show(current + 1);
    timer = setTimeout(tick, DELAY);
  }

  function play() {
    if (current >= steps.length - 1) show(0);
    playButton.textContent = '⏸';
    clearTimeout(timer);
    timer = setTimeout(tick, DELAY);
  }

  function toggle() {
    if (timer) pause();
    else play();
  }

  const actions = {
    first: function () { pause(); show(0); },
    previous: function () { pause(); show(current - 1); },
    play: toggle,
    next: function () { pause(); show(current + 1); },
    last: function () { pause(); show(steps.length - 1); },
  };
  document.querySelectorAll('[data-replay-action]').forEach(function (button) {
    button.addEventListener('click', actions[button.dataset.replayAction]);
  });
  progress.max = steps.length - 1;
  progress.addEventListener('input', function () {
    pause();
    show(Number(progress.value));
  });
  document.addEventListener('keydown', function (event) {
    if (event.target.closest('input, select, textarea, button')) return;
    if (event.key === ' ') {
      event.preventDefault();
      toggle();
    } else if (event.key === 'ArrowLeft') {
      actions.previous();
    } else if (event.key === 'ArrowRight') {
      actions.next();
    }
  });

  buildLog();
  show(0);
  play();
})();
