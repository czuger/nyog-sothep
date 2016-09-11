# Place all the behaviors and hooks related to the matching controller here.
# All this logic will automatically be available in application.js.
# You can use CoffeeScript in this file: http://coffeescript.org/

actions_init = ->

  switch_table = ->
    $.post '/maps/switch_table',
      event_table: $(this)[0].value

  $( '.event_table' ).change( switch_table )

#$(document).ready(actions_init) # otherwise the event is called twice after page loading
$(document).on('turbolinks:load', actions_init)