module GameCore
  # To be inclued in GGameBoard
  module NyogSothep

    def nyog_sothep_repelled_test
      investigators_on_nyog_sothep_city = nyog_sothep_encounter
      # If nyog sothep is at the repelling place
      if nyog_sothep_current_location_code_name == nyog_sothep_repelling_city_code_name
        # Do we have 3 investigators with the spell at this place ?
        investigators_count = investigators_on_nyog_sothep_city.count
        if investigators_count >= 3
          san_loss = nil
          case repelling_roll( investigators_count )
            when 3
              san_loss = 4
            when 2
              san_loss = 2
            when 0, 1
              # Nothing happens
            when -1
              # TODO : Move Nyog Sothep randomly
            else
              # TODO : implement victory
          end

          if san_loss
            investigators_on_nyog_sothep_city.each do |i|
              event_log = EEventLog.log( self, i, 'Le renvoi de Nyog Sothep échoue' )
              i.loose_san( self, san_loss, event_log )
            end
          end

        end
      end
    end

    # Return true if Nyog Sothep can be invoked
    def check_nyog_sothep_invocation( prof )
      unless nyog_sothep_invoked
        nb_fanatiques = p_monster_positions.where( code_name: 'fanatiques' ).count()
        if nb_fanatiques >= 5

          return true if prof.current_location_code_name == nyog_sothep_invocation_position_code_name

          _, dist_to_nyog = GameCore::Ia::BfsAlgo.find_next_dest_to_goal(
            prof.current_location_code_name, nyog_sothep_invocation_position_code_name )
          return true if dist_to_nyog <= 3
        end
      end

      false
    end

    # Return true if nyog sothep not invoked, nyog sothep not with prof or nyog sothep allowed to move
    def move_nyog_sothep(  prof, dest_loc )
      if nyog_sothep_invoked
        raise "Nyog Sothep current location should not be nil" unless nyog_sothep_current_location_code_name

        if self.nyog_sothep_current_location_code_name == prof.current_location_code_name

          if rand( 1 .. 6 ) <= 2
            # Nyog sothep and the professor are forbidden to move
            event_log = EEventLog.log( self, prof, I18n.t( 'map.event_log_summaries.nyog_cant_move' ) )
            EEventLogSummary.log( self, prof, :nyog_cant_move, {}, event_log )
            return false
          end

          self.nyog_sothep_current_location_code_name = dest_loc.code_name

          GDestroyedCity.destroy_city( self, dest_loc.code_name )
        end
      end
      true
    end

    private

    # Separate this will be usefull for the tests
    def repelling_roll( investigators_count )
      rand( 1 .. 6 ) - investigators_count
    end

    def nyog_sothep_encounter
      investigators_on_nyog_sothep_city = alive_investigators.where( spell: true ).where( current_location_code_name: nyog_sothep_current_location_code_name )
      investigators_on_nyog_sothep_city.each do |i|
        if i.nyog_sothep_already_seen
          event_log = EEventLog.log( self, i, 'Nyog sothep est vu pour la première fois' )
          i.loose_san( self, 2, event_log )
        else
          event_log = EEventLog.log( self, i, 'Nyog sothep est vu à nouveau' )
          i.nyog_sothep_already_seen = true
          i.loose_san( self, 4, event_log )
        end
      end
      investigators_on_nyog_sothep_city
    end

  end
end
