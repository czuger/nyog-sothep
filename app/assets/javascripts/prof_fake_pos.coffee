# Place all the behaviors and hooks related to the matching controller here.
# All this logic will automatically be available in application.js.
# You can use CoffeeScript in this file: http://coffeescript.org/

selected_cities_cnt = 0

cities_validation = ->

  $( '#prof-fake-position-submit-button' ).click ->
    cities_ids = []

    for d in $( "div[selk='true']" )
#      console.log( d )
      cities_ids.push( $( d ).attr( 'city_id' ) )

    gb_id = $( '#game_board_id' ).val()
    $.post "/g_game_boards/#{gb_id}/prof_fake_pos", cities_ids: cities_ids

enable_disable_submit_button = (max_selected_cities_cnt) ->

  if selected_cities_cnt == max_selected_cities_cnt
    $( '#prof-fake-position-submit-button' ).prop('disabled', false)
  else
    $( '#prof-fake-position-submit-button' ).prop('disabled', true)


city_selection_init = ->
  max_selected_cities_cnt = parseInt( $( '#nb_cities' ).val() )

  if $( '#last_fake_positions_codes_names' ).length != 0
    selected_cities_cnt = 0
    selected_cities = JSON.parse( $( '#last_fake_positions_codes_names' ).val() )
    console.log('selected_cities',selected_cities)

    for city in selected_cities
      cities = $.find("[city_id='" + city + "']")
      for city in cities
        city = $(city)
        city.css( 'border', 'solid black' )
        city.attr( 'selk', 'true' )
        selected_cities_cnt += 1

        break if selected_cities_cnt >= max_selected_cities_cnt
      break if selected_cities_cnt >= max_selected_cities_cnt

    enable_disable_submit_button(max_selected_cities_cnt)

city_selection = ->
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

    enable_disable_submit_button(max_selected_cities_cnt)


$(document).on('turbolinks:load', city_selection_init)
$(document).on('turbolinks:load', city_selection)
$(document).on('turbolinks:load', cities_validation)