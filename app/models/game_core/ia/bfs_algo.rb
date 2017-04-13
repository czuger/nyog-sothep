module GameCore
  module Ia

    class BfsAlgo
      
      DESTINATIONS = {'plainfield'=>['providence', 'oxford', 'westerly'],
                      'pascoag'=>['woonsocket', 'oxford'],
                      'woonsocket'=>['pascoag', 'providence', 'attleboro', 'dedham', 'milford'],
                      'providence'=>
                        ['plainfield',
                         'woonsocket',
                         'attleboro',
                         'apponaug',
                         'bristol',
                         'fall_river'],
                      'foxboro'=>['attleboro', 'taunton', 'stoughton'],
                      'attleboro'=>['woonsocket', 'providence', 'foxboro', 'taunton'],
                      'taunton'=>['foxboro', 'attleboro', 'middleboro', 'stoughton', 'fall_river'],
                      'rockland'=>['bridgewater_junction', 'greenbush', 'boston'],
                      'bridgewater_junction'=>['rockland', 'middleboro', 'boston'],
                      'middleboro'=>
                        ['taunton', 'bridgewater_junction', 'tremont', 'new_bedford', 'plymouth'],
                      'greenbush'=>['rockland', 'plymouth'],
                      'tremont'=>['middleboro', 'falmouth', 'barnstable', 'plymouth'],
                      'falmouth'=>['tremont', 'barnstable'],
                      'barnstable'=>['tremont', 'falmouth', 'chatham', 'provincetown'],
                      'worcester'=>['oxford', 'westboro', 'milford', 'oakdale'],
                      'oxford'=>['plainfield', 'pascoag', 'worcester'],
                      'westboro'=>['worcester', 'ashland'],
                      'ashland'=>['westboro', 'milford', 'sudbury', 'boston'],
                      'dedham'=>['woonsocket', 'stoughton', 'boston'],
                      'milford'=>['woonsocket', 'worcester', 'ashland'],
                      'stoughton'=>['foxboro', 'taunton', 'dedham'],
                      'sudbury'=>['ashland', 'oakdale', 'concord', 'boston'],
                      'nashua'=>['dunwich', 'lowel'],
                      'dunwich'=>['nashua', 'lowel', 'fitchburg', 'concord'],
                      'lowel'=>['nashua', 'dunwich', 'fitchburg', 'concord', 'lawrence'],
                      'fitchburg'=>['dunwich', 'lowel', 'oakdale', 'concord'],
                      'oakdale'=>['worcester', 'sudbury', 'fitchburg'],
                      'concord'=>['sudbury', 'dunwich', 'lowel', 'fitchburg', 'boston'],
                      'lawrence'=>['lowel', 'newburyport', 'arkham'],
                      'newburyport'=>['lawrence', 'innsmouth', 'arkham', 'gulf_of_maine'],
                      'innsmouth'=>['newburyport', 'arkham', 'gulf_of_maine'],
                      'gloucester'=>['gulf_of_maine'],
                      'arkham'=>
                        ['lawrence', 'newburyport', 'innsmouth', 'lynn', 'massachusetts_bay'],
                      'lynn'=>['arkham', 'boston', 'massachusetts_bay'],
                      'boston'=>
                        ['rockland',
                         'bridgewater_junction',
                         'ashland',
                         'dedham',
                         'sudbury',
                         'concord',
                         'lynn',
                         'massachusetts_bay'],
                      'westerly'=>['plainfield', 'narragansett_pier', 'atlantic_ocean'],
                      'narragansett_pier'=>['westerly', 'apponaug', 'atlantic_ocean'],
                      'apponaug'=>['providence', 'narragansett_pier', 'atlantic_ocean'],
                      'bristol'=>['providence', 'fall_river', 'atlantic_ocean'],
                      'newport'=>['fall_river', 'atlantic_ocean'],
                      'fall_river'=>
                        ['providence',
                         'taunton',
                         'bristol',
                         'newport',
                         'new_bedford',
                         'atlantic_ocean'],
                      'new_bedford'=>['middleboro', 'fall_river', 'buzzards_bay'],
                      'edgartown'=>['nantucket_sound'],
                      'nantucket'=>['nantucket_sound'],
                      'chatham'=>['barnstable', 'provincetown', 'nantucket_sound'],
                      'plymouth'=>['middleboro', 'greenbush', 'tremont', 'cape_cod_bay'],
                      'provincetown'=>['barnstable', 'chatham', 'cape_cod_bay'],
                      'gulf_of_maine'=>
                        ['newburyport',
                         'innsmouth',
                         'gloucester',
                         'massachusetts_bay',
                         'nantucket_sound',
                         'cape_cod_bay'],
                      'massachusetts_bay'=>
                        ['arkham', 'lynn', 'boston', 'gulf_of_maine', 'cape_cod_bay'],
                      'atlantic_ocean'=>
                        ['westerly',
                         'narragansett_pier',
                         'apponaug',
                         'bristol',
                         'newport',
                         'fall_river',
                         'buzzards_bay',
                         'nantucket_sound'],
                      'buzzards_bay'=>['new_bedford', 'atlantic_ocean', 'nantucket_sound'],
                      'nantucket_sound'=>
                        ['edgartown',
                         'nantucket',
                         'chatham',
                         'gulf_of_maine',
                         'atlantic_ocean',
                         'buzzards_bay'],
                      'cape_cod_bay'=>
                        ['plymouth', 'provincetown', 'gulf_of_maine', 'massachusetts_bay']}
      
      DEST_KLASS = {'plainfield'=>'CCity',
                    'pascoag'=>'CCity',
                    'woonsocket'=>'CCity',
                    'providence'=>'CCity',
                    'foxboro'=>'CCity',
                    'attleboro'=>'CCity',
                    'taunton'=>'CCity',
                    'rockland'=>'CCity',
                    'bridgewater_junction'=>'CCity',
                    'middleboro'=>'CCity',
                    'greenbush'=>'CCity',
                    'tremont'=>'CCity',
                    'falmouth'=>'CCity',
                    'barnstable'=>'CCity',
                    'worcester'=>'CCity',
                    'oxford'=>'CCity',
                    'westboro'=>'CCity',
                    'ashland'=>'CCity',
                    'dedham'=>'CCity',
                    'milford'=>'CCity',
                    'stoughton'=>'CCity',
                    'sudbury'=>'CCity',
                    'nashua'=>'CCity',
                    'dunwich'=>'CCity',
                    'lowel'=>'CCity',
                    'fitchburg'=>'CCity',
                    'oakdale'=>'CCity',
                    'concord'=>'CCity',
                    'lawrence'=>'CCity',
                    'newburyport'=>'CCity',
                    'innsmouth'=>'CCity',
                    'gloucester'=>'CCity',
                    'arkham'=>'CCity',
                    'lynn'=>'CCity',
                    'boston'=>'CCity',
                    'westerly'=>'CCity',
                    'narragansett_pier'=>'CCity',
                    'apponaug'=>'CCity',
                    'bristol'=>'CCity',
                    'newport'=>'CCity',
                    'fall_river'=>'CCity',
                    'new_bedford'=>'CCity',
                    'edgartown'=>'CCity',
                    'nantucket'=>'CCity',
                    'chatham'=>'CCity',
                    'plymouth'=>'CCity',
                    'provincetown'=>'CCity',
                    'gulf_of_maine'=>'WWaterArea',
                    'massachusetts_bay'=>'WWaterArea',
                    'atlantic_ocean'=>'WWaterArea',
                    'buzzards_bay'=>'WWaterArea',
                    'nantucket_sound'=>'WWaterArea',
                    'cape_cod_bay'=>'WWaterArea'}

      def self.find_next_dest_to_goal( current_position, goal )

        goal_name = goal.code_name

        frontier = []
        frontier << current_position.code_name
        came_from = {}
        came_from[ current_position.code_name ] = nil

        until frontier.empty?
          current_name = frontier.shift

          if current_name == goal_name
            break
          end

          DESTINATIONS[ current_name ].each do |next_location|
            unless came_from.has_key?( next_location )
              frontier << next_location
              came_from[ next_location ] = current_name
            end
          end
        end

        back_token = goal_name
        next_step = nil

        while back_token != current_position.code_name
          next_step = back_token
          back_token = came_from[ back_token ]
          # p next_step
          break if next_step == nil
        end

        DEST_KLASS[ next_step ].constantize.find_by( code_name: next_step )
      end

      def self.create_dest_hash

        dest_hash = {}
        class_hash = {}

        CCity.all.each do |c|
          dest_hash[ c.code_name ] = c.destinations.map{ |e| e.code_name }
          class_hash[ c.code_name ] = c.class.to_s
        end

        WWaterArea.all.each do |c|
          dest_hash[ c.code_name ] = c.destinations.map{ |e| e.code_name }
          class_hash[ c.code_name ] = c.class.to_s
        end

        pp dest_hash
        pp class_hash

      end
    end
  end
end
