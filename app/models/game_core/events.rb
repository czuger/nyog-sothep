# To be included in investigator
module GameCore
  module Events

    include GameCore::EventGroundA
    include GameCore::EventGroundB

    def roll_event( game_board, professor, prof_position_finder )
      if current_location.city? && last_location.city?

        table = choose_table
        roll = event_dices

        #  For dev
        # table = 1
        # roll = 18

        # LLog.log( game_board, self, "table#{table}_e#{roll}" )

        send( "table#{table}_e#{roll}", game_board, professor, prof_position_finder )
      else
        LLog.log( game_board, self, 'events.no_water_events' )
      end
    end

    def method_missing( method_name, *args )
      super unless method_name =~ /table.*/

      unless args.first
        raise "Bad arguements : method_name = #{method_name}, args = #{args.inspect}"
      end

      LLog.log( args.first, self, 'events.not_implemented', false, { method: method_name } )
    end

    def choose_table
      Kernel.rand( 1 ..2 )
    end

    def event_dices
      roll = GameCore::Dices.d6
      roll += GameCore::Dices.d6 if sign
      roll += GameCore::Dices.d6 if sign && medaillon
      roll
    end

    private

    def ask_prof_for_fake_cities( game_board, nb_cities )
      game_board.update( asked_fake_cities_count: nb_cities, asked_fake_cities_investigator: self )
      game_board.ask_prof_for_fake_cities!
    end

    def loose_san_from_event( game_board, method, item, amount )
      update_attribute( item, true ) if loose_san( game_board,  amount )
      log_event( game_board, method, { san_loss: amount, cur_san: san } )
    end

    def log_event( game_board, method, san_hash = {} )

      summary = san_hash.has_key?( :san_loss ) || san_hash.has_key?( :san_gain )

      log_event_code = 'events.' + method.to_s.gsub('_','.')
      name_translation_method = I18n.t( log_event_code + '.name_translation' )

      LLog.log( game_board, self, log_event_code, summary, san_hash,
                true, false, name_translation_method )
    end
  end
end