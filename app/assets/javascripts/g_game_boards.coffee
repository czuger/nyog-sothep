# Place all the behaviors and hooks related to the matching controller here.
# All this logic will automatically be available in application.js.
# You can use CoffeeScript in this file: http://coffeescript.org/



prof_and_nyog_drag_n_drop = ->

  prof_position = null
  nyog_sothep_position = null

  $( '#game-board-creation-button' ).click ->
    $.post "/g_game_boards", prof_position_code_name: prof_position, nyog_sothep_position_code_name: nyog_sothep_position

  $( ".drag-drop-icons" ).draggable()

  $( ".prof-nyog-positions" ).droppable
    accept: ".drag-drop-icons"
    drop: (event,ui) ->
#      console.log( event )
#      console.log( ui.draggable )
#      console.log( $(this) )

      $(this).append( ui.draggable )

      ui.draggable.css("top", 0);
      ui.draggable.css("left", 0);

      if ui.draggable.hasClass( 'professor' )
        prof_position = $(this).attr( 'city_id' )
      else
        nyog_sothep_position = $(this).attr( 'city_id' )

      console.log( prof_position )
      console.log( nyog_sothep_position )

# $(document).ready(scroll_screen_to_lowest_elem)
$(document).on('turbolinks:load', prof_and_nyog_drag_n_drop)
