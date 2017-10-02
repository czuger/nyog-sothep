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
  ),
    ->
      log_id = $(this).attr( 'log_id' )
      $( "#log-summary-detail-#{log_id}" ).hide()
#

investigator_informations_hoovering = ->
  $('.investigator-token').hover (->
      inv_id = $(this).attr( 'inv_id' )
      $( "#investigator-#{inv_id}-info" ).show()
    ),
    ->
      inv_id = $(this).attr( 'inv_id' )
      $( "#investigator-#{inv_id}-info" ).hide()

monster_token_on_map_hoovering = ->
  $('.monster-token').hover (->
    m_id = $(this).attr( 'monster_id' )
    $( "#monster-#{m_id}-info" ).show()
  ),
    ->
      m_id = $(this).attr( 'monster_id' )
      $( "#monster-#{m_id}-info" ).hide()

monster_selector_hoovering = ->
  $('.monster-picture').hover (->
    m_id = $(this).attr( 'monster_id' )
    console.log('monster_selector_hoovering', m_id)

    position = $(this).position()
    $( "#monster-selector-#{m_id}-info" ).css('top',position.top-70)
    $( "#monster-selector-#{m_id}-info" ).css('left',position.left-100)
    $( "#monster-selector-#{m_id}-info" ).css('z-index',50)
    $( "#monster-selector-#{m_id}-info" ).show()
  ),
    ->
      m_id = $(this).attr( 'monster_id' )
      $( "#monster-selector-#{m_id}-info" ).hide()

$(window).load ->
  investigator_informations_hoovering()
  log_summary_hoovering()
  monster_token_on_map_hoovering()
  monster_selector_hoovering()
