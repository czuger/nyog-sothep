# Place all the behaviors and hooks related to the matching controller here.
# All this logic will automatically be available in application.js.
# You can use CoffeeScript in this file: http://coffeescript.org/

cities_validation = ->

  $( '#prof-fake-position-submit-button' ).click ->
    cities_ids = []

    for d in $( "div[selk='true']" )
#      console.log( d )
      cities_ids.push( $( d ).attr( 'city_id' ) )

    gb_id = $( '#game_board_id' ).val()
    $.post "/g_game_boards/#{gb_id}/prof_fake_pos", cities_ids: cities_ids

city_selection = ->

  selected_cities_cnt = 0
  max_selected_cities_cnt = parseInt( $( '#nb_cities' ).val() )

  $( '.prof-fake-position' ).click ->
    if $(this).attr( 'selk' ) == 'true'

      $(this).css( 'border', '' )
      $(this).attr( 'selk', 'false' )
      selected_cities_cnt -= 1

    else
      if selected_cities_cnt < max_selected_cities_cnt

        $(this).css( 'border', 'solid black' )
        $(this).attr( 'selk', 'true' )
        selected_cities_cnt += 1

    if selected_cities_cnt == max_selected_cities_cnt
      $( '#prof-fake-position-submit-button' ).prop('disabled', false)
    else
      $( '#prof-fake-position-submit-button' ).prop('disabled', true)

$(document).on('turbolinks:load', city_selection)
$(document).on('turbolinks:load', cities_validation)