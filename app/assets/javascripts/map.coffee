# Place all the behaviors and hooks related to the matching controller here.
# All this logic will automatically be available in application.js.
# You can use CoffeeScript in this file: http://coffeescript.org/

# TODO : need to compute
ready = ->
  lowest_elem = $( '#professor' )
  lowest_position = lowest_elem.position().top

  for elem in $( '.dest-road' )
    top = $( elem ).position().top
    console.log( top )
    if top > lowest_position
      lowest_position = top
      lowest_elem = $( elem )

  for elem in $( '.dest-water' )
    top = $( elem ).position().top
    console.log( top )
    if top > lowest_position
      lowest_position = top
      lowest_elem = $( elem )

  lowest_elem[0].scrollIntoView( false )

$(document).ready(ready)
$(document).on('turbolinks:load', ready)
