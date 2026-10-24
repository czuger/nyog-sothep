// The map: scaled to the room left by the floating panels, with investigator moves staged by clicking.
// A mover ([data-mover]) names the select of its move (data-mover) and lists in data-targets the
// [data-target] places it may enter; staging a move sets that select and draws an arrow.
(function () {
  const board = document.querySelector('.board');
  if (!board) return;
  const SVG_NS = 'http://www.w3.org/2000/svg';
  const boardWidth = board.offsetWidth;
  const viewport = board.closest('.board-viewport');
  const arrows = board.querySelector('.board-moves');
  let selected = null;

  // The board keeps its pixel coordinates and is scaled as a whole to the width left between the
  // open floating panels, so that none of them hides a city; the map scrolls vertically.
  function fitBoard() {
    const gap = 8;
    const viewportWidth = viewport.clientWidth;
    let left = 0;
    let right = viewportWidth;
    document.querySelectorAll('.floating[open], .floating-column').forEach(function (panel) {
      const rect = panel.getBoundingClientRect();
      if (rect.width === 0 || viewportWidth < 760) return;
      if (rect.left < viewportWidth / 2) left = Math.max(left, rect.right + gap);
      else right = Math.min(right, rect.left - gap);
    });
    const width = Math.max(right - left, 200);
    viewport.style.setProperty('--board-scale', width / boardWidth);
    viewport.style.setProperty('--board-center', left + width / 2 + 'px');
  }

  // The page reloads after every answer: keep the panels as the player left them.
  function rememberPanels() {
    document.querySelectorAll('details[data-remember]').forEach(function (panel) {
      const key = 'nyog-panel-' + panel.dataset.remember;
      try {
        const saved = localStorage.getItem(key);
        if (saved !== null) panel.open = saved === 'open';
      } catch (error) {
        // Storage unavailable: panels keep their default state.
      }
      panel.addEventListener('toggle', function () {
        fitBoard();
        try {
          localStorage.setItem(key, panel.open ? 'open' : 'closed');
        } catch (error) {
          // Storage unavailable: the state is simply not remembered.
        }
      });
    });
  }

  // The map scrolls inside its viewport, which the browser does not restore on reload. The key is the
  // game's path, shared by the error page shown at /games/{id}/{token}/{prompt}; the position is kept as a
  // fraction of the height so that a different board scale lands on the same part of the map.
  function rememberScroll() {
    const key = 'nyog-scroll-' + location.pathname.split('/').slice(0, 4).join('/');
    try {
      const saved = parseFloat(sessionStorage.getItem(key));
      if (!Number.isNaN(saved)) viewport.scrollTop = saved * viewport.scrollHeight;
    } catch (error) {
      // Storage unavailable: the map opens at the top.
    }
    window.addEventListener('pagehide', function () {
      try {
        sessionStorage.setItem(key, String(viewport.scrollTop / viewport.scrollHeight));
      } catch (error) {
        // Storage unavailable: the position is simply not remembered.
      }
    });
  }

  function place(code) {
    return board.querySelector('[data-target="' + code + '"]');
  }

  function selectFor(mover) {
    return document.querySelector('select[name="' + mover.dataset.mover + '"]');
  }

  function select(mover) {
    document.body.classList.toggle('choosing', Boolean(mover));
    if (selected) selected.classList.remove('is-selected');
    board.querySelectorAll('.is-target').forEach(function (element) {
      element.classList.remove('is-target');
    });
    selected = mover;
    if (!mover) return;
    mover.classList.add('is-selected');
    mover.dataset.targets.split(' ').forEach(function (code) {
      const element = place(code);
      if (element) element.classList.add('is-target');
    });
  }

  // Arrows always mirror the move selects, whether set on the map or in the panel.
  function drawArrows() {
    if (!arrows) return;
    arrows.querySelectorAll('line').forEach(function (line) { line.remove(); });
    board.querySelectorAll('[data-mover]').forEach(function (mover) {
      const choice = selectFor(mover);
      const moving = choice && place(choice.value);
      mover.classList.toggle('is-staged', Boolean(choice) && choice.value !== 'stay');
      if (!moving) return;
      const start = place(mover.dataset.from);
      const line = document.createElementNS(SVG_NS, 'line');
      line.setAttribute('x1', start.dataset.x);
      line.setAttribute('y1', start.dataset.y);
      line.setAttribute('x2', moving.dataset.x);
      line.setAttribute('y2', moving.dataset.y);
      line.setAttribute('marker-end', 'url(#move-arrow)');
      arrows.appendChild(line);
    });
  }

  function stage(code) {
    const choice = selectFor(selected);
    if (choice) choice.value = code;
    drawArrows();
    select(null);
  }

  board.querySelectorAll('[data-mover]').forEach(function (mover) {
    mover.addEventListener('click', function (event) {
      event.stopPropagation();
      // Another investigator standing on a destination: the click means that destination.
      const reachable = selected && selected !== mover && selected.dataset.targets.split(' ').includes(mover.dataset.from);
      if (reachable) stage(mover.dataset.from);
      else select(selected === mover ? null : mover);
    });
  });

  board.addEventListener('click', function (event) {
    if (!selected) return;
    const target = event.target.closest('.is-target');
    if (target) stage(target.dataset.target);
    else select(null);
  });

  document.querySelectorAll('[data-move-select]').forEach(function (choice) {
    choice.addEventListener('change', drawArrows);
  });

  document.addEventListener('keydown', function (event) {
    if (event.key === 'Escape') select(null);
  });

  rememberPanels();
  fitBoard();
  rememberScroll();
  drawArrows();
  window.addEventListener('resize', fitBoard);
})();
