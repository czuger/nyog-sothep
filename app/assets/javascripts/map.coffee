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
