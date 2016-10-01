# Place all the behaviors and hooks related to the matching controller here.
# All this logic will automatically be available in application.js.
# You can use CoffeeScript in this file: http://coffeescript.org/

actions_init = ->

  switch_table = ->

    game_board_id = $( '#game_board_id' ).val()
    investigator_id = $( '#investigator_id' ).val()

    $.post "/g_game_boards/#{game_board_id}/investigators_actions/#{investigator_id}/switch_table",
      event_table: $(this)[0].value

  $( '.event_table' ).change( switch_table )

#$(document).ready(actions_init) # otherwise the event is called twice after page loading
$(document).on('turbolinks:load', actions_init)