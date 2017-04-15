# Place all the behaviors and hooks related to the matching controller here.
# All this logic will automatically be available in application.js.
# You can use CoffeeScript in this file: http://coffeescript.org/

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

$(document).on('turbolinks:load', city_selection)