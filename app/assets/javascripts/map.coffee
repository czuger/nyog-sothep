# Place all the behaviors and hooks related to the matching controller here.
# All this logic will automatically be available in application.js.
# You can use CoffeeScript in this file: http://coffeescript.org/

#scroll_screen_to_lowest_elem = ->
#  lowest_elem = $( '.token' )
#  lowest_position = lowest_elem.position().top
#
#  for elem in $( '.dest-road' )
#    top = $( elem ).position().top
#    #console.log( top )
#    if top > lowest_position
#      lowest_position = top
#      lowest_elem = $( elem )
#
#  for elem in $( '.dest-water' )
#    top = $( elem ).position().top
#    #console.log( top )
#    if top > lowest_position
#      lowest_position = top
#      lowest_elem = $( elem )
#
#  lowest_elem[0].scrollIntoView( false )
#
## $(document).ready(scroll_screen_to_lowest_elem)
#$(document).on('turbolinks:load', scroll_screen_to_lowest_elem)

log_summary_hoovering = ->
  $('.log-summary-detail-hoover').hover (->
    log_id = $(this).attr( 'log_id' )
    $( "#log-summary-detail-#{log_id}" ).show()
#      console.log( "hoover : #{inv_id}" )
  ),
    ->
      log_id = $(this).attr( 'log_id' )
      $( "#log-summary-detail-#{log_id}" ).hide()
#

investigator_informations_hoovering = ->
  $('.investigator-token').hover (->
      inv_id = $(this).attr( 'inv_id' )
      $( "#investigator-#{inv_id}-info" ).show()
#      console.log( "hoover : #{inv_id}" )
    ),
    ->
      inv_id = $(this).attr( 'inv_id' )
      $( "#investigator-#{inv_id}-info" ).hide()
#      console.log( 'unhoover' )

$(document).on('turbolinks:load', investigator_informations_hoovering)
$(document).on('turbolinks:load', log_summary_hoovering)
