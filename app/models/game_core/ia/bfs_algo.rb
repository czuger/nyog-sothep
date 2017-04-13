module GameCore
  module Ia

    class BfsAlgo
      
      DESTINATIONS = {
       'plainfield'=>
         [{:code_name=>'providence', :klass=>'CCity'},
          {:code_name=>'oxford', :klass=>'CCity'},
          {:code_name=>'westerly', :klass=>'CCity'}],
       'pascoag'=>
         [{:code_name=>'woonsocket', :klass=>'CCity'},
          {:code_name=>'oxford', :klass=>'CCity'}],
       'woonsocket'=>
         [{:code_name=>'pascoag', :klass=>'CCity'},
          {:code_name=>'providence', :klass=>'CCity'},
          {:code_name=>'attleboro', :klass=>'CCity'},
          {:code_name=>'dedham', :klass=>'CCity'},
          {:code_name=>'milford', :klass=>'CCity'}],
       'providence'=>
         [{:code_name=>'plainfield', :klass=>'CCity'},
          {:code_name=>'woonsocket', :klass=>'CCity'},
          {:code_name=>'attleboro', :klass=>'CCity'},
          {:code_name=>'apponaug', :klass=>'CCity'},
          {:code_name=>'bristol', :klass=>'CCity'},
          {:code_name=>'fall_river', :klass=>'CCity'}],
       'foxboro'=>
         [{:code_name=>'attleboro', :klass=>'CCity'},
          {:code_name=>'taunton', :klass=>'CCity'},
          {:code_name=>'stoughton', :klass=>'CCity'}],
       'attleboro'=>
         [{:code_name=>'woonsocket', :klass=>'CCity'},
          {:code_name=>'providence', :klass=>'CCity'},
          {:code_name=>'foxboro', :klass=>'CCity'},
          {:code_name=>'taunton', :klass=>'CCity'}],
       'taunton'=>
         [{:code_name=>'foxboro', :klass=>'CCity'},
          {:code_name=>'attleboro', :klass=>'CCity'},
          {:code_name=>'middleboro', :klass=>'CCity'},
          {:code_name=>'stoughton', :klass=>'CCity'},
          {:code_name=>'fall_river', :klass=>'CCity'}],
       'rockland'=>
         [{:code_name=>'bridgewater_junction', :klass=>'CCity'},
          {:code_name=>'greenbush', :klass=>'CCity'},
          {:code_name=>'boston', :klass=>'CCity'}],
       'bridgewater_junction'=>
         [{:code_name=>'rockland', :klass=>'CCity'},
          {:code_name=>'middleboro', :klass=>'CCity'},
          {:code_name=>'boston', :klass=>'CCity'}],
       'middleboro'=>
         [{:code_name=>'taunton', :klass=>'CCity'},
          {:code_name=>'bridgewater_junction', :klass=>'CCity'},
          {:code_name=>'tremont', :klass=>'CCity'},
          {:code_name=>'new_bedford', :klass=>'CCity'},
          {:code_name=>'plymouth', :klass=>'CCity'}],
       'greenbush'=>
         [{:code_name=>'rockland', :klass=>'CCity'},
          {:code_name=>'plymouth', :klass=>'CCity'}],
       'tremont'=>
         [{:code_name=>'middleboro', :klass=>'CCity'},
          {:code_name=>'falmouth', :klass=>'CCity'},
          {:code_name=>'barnstable', :klass=>'CCity'},
          {:code_name=>'plymouth', :klass=>'CCity'}],
       'falmouth'=>
         [{:code_name=>'tremont', :klass=>'CCity'},
          {:code_name=>'barnstable', :klass=>'CCity'}],
       'barnstable'=>
         [{:code_name=>'tremont', :klass=>'CCity'},
          {:code_name=>'falmouth', :klass=>'CCity'},
          {:code_name=>'chatham', :klass=>'CCity'},
          {:code_name=>'provincetown', :klass=>'CCity'}],
       'worcester'=>
         [{:code_name=>'oxford', :klass=>'CCity'},
          {:code_name=>'westboro', :klass=>'CCity'},
          {:code_name=>'milford', :klass=>'CCity'},
          {:code_name=>'oakdale', :klass=>'CCity'}],
       'oxford'=>
         [{:code_name=>'plainfield', :klass=>'CCity'},
          {:code_name=>'pascoag', :klass=>'CCity'},
          {:code_name=>'worcester', :klass=>'CCity'}],
       'westboro'=>
         [{:code_name=>'worcester', :klass=>'CCity'},
          {:code_name=>'ashland', :klass=>'CCity'}],
       'ashland'=>
         [{:code_name=>'westboro', :klass=>'CCity'},
          {:code_name=>'milford', :klass=>'CCity'},
          {:code_name=>'sudbury', :klass=>'CCity'},
          {:code_name=>'boston', :klass=>'CCity'}],
       'dedham'=>
         [{:code_name=>'woonsocket', :klass=>'CCity'},
          {:code_name=>'stoughton', :klass=>'CCity'},
          {:code_name=>'boston', :klass=>'CCity'}],
       'milford'=>
         [{:code_name=>'woonsocket', :klass=>'CCity'},
          {:code_name=>'worcester', :klass=>'CCity'},
          {:code_name=>'ashland', :klass=>'CCity'}],
       'stoughton'=>
         [{:code_name=>'foxboro', :klass=>'CCity'},
          {:code_name=>'taunton', :klass=>'CCity'},
          {:code_name=>'dedham', :klass=>'CCity'}],
       'sudbury'=>
         [{:code_name=>'ashland', :klass=>'CCity'},
          {:code_name=>'oakdale', :klass=>'CCity'},
          {:code_name=>'concord', :klass=>'CCity'},
          {:code_name=>'boston', :klass=>'CCity'}],
       'nashua'=>
         [{:code_name=>'dunwich', :klass=>'CCity'},
          {:code_name=>'lowel', :klass=>'CCity'}],
       'dunwich'=>
         [{:code_name=>'nashua', :klass=>'CCity'},
          {:code_name=>'lowel', :klass=>'CCity'},
          {:code_name=>'fitchburg', :klass=>'CCity'},
          {:code_name=>'concord', :klass=>'CCity'}],
       'lowel'=>
         [{:code_name=>'nashua', :klass=>'CCity'},
          {:code_name=>'dunwich', :klass=>'CCity'},
          {:code_name=>'fitchburg', :klass=>'CCity'},
          {:code_name=>'concord', :klass=>'CCity'},
          {:code_name=>'lawrence', :klass=>'CCity'}],
       'fitchburg'=>
         [{:code_name=>'dunwich', :klass=>'CCity'},
          {:code_name=>'lowel', :klass=>'CCity'},
          {:code_name=>'oakdale', :klass=>'CCity'},
          {:code_name=>'concord', :klass=>'CCity'}],
       'oakdale'=>
         [{:code_name=>'worcester', :klass=>'CCity'},
          {:code_name=>'sudbury', :klass=>'CCity'},
          {:code_name=>'fitchburg', :klass=>'CCity'}],
       'concord'=>
         [{:code_name=>'sudbury', :klass=>'CCity'},
          {:code_name=>'dunwich', :klass=>'CCity'},
          {:code_name=>'lowel', :klass=>'CCity'},
          {:code_name=>'fitchburg', :klass=>'CCity'},
          {:code_name=>'boston', :klass=>'CCity'}],
       'lawrence'=>
         [{:code_name=>'lowel', :klass=>'CCity'},
          {:code_name=>'newburyport', :klass=>'CCity'},
          {:code_name=>'arkham', :klass=>'CCity'}],
       'newburyport'=>
         [{:code_name=>'lawrence', :klass=>'CCity'},
          {:code_name=>'innsmouth', :klass=>'CCity'},
          {:code_name=>'arkham', :klass=>'CCity'},
          {:code_name=>'gulf_of_maine', :klass=>'WWaterArea'}],
       'innsmouth'=>
         [{:code_name=>'newburyport', :klass=>'CCity'},
          {:code_name=>'arkham', :klass=>'CCity'},
          {:code_name=>'gulf_of_maine', :klass=>'WWaterArea'}],
       'gloucester'=>[{:code_name=>'gulf_of_maine', :klass=>'WWaterArea'}],
       'arkham'=>
         [{:code_name=>'lawrence', :klass=>'CCity'},
          {:code_name=>'newburyport', :klass=>'CCity'},
          {:code_name=>'innsmouth', :klass=>'CCity'},
          {:code_name=>'lynn', :klass=>'CCity'},
          {:code_name=>'massachusetts_bay', :klass=>'WWaterArea'}],
       'lynn'=>
         [{:code_name=>'arkham', :klass=>'CCity'},
          {:code_name=>'boston', :klass=>'CCity'},
          {:code_name=>'massachusetts_bay', :klass=>'WWaterArea'}],
       'boston'=>
         [{:code_name=>'rockland', :klass=>'CCity'},
          {:code_name=>'bridgewater_junction', :klass=>'CCity'},
          {:code_name=>'ashland', :klass=>'CCity'},
          {:code_name=>'dedham', :klass=>'CCity'},
          {:code_name=>'sudbury', :klass=>'CCity'},
          {:code_name=>'concord', :klass=>'CCity'},
          {:code_name=>'lynn', :klass=>'CCity'},
          {:code_name=>'massachusetts_bay', :klass=>'WWaterArea'}],
       'westerly'=>
         [{:code_name=>'plainfield', :klass=>'CCity'},
          {:code_name=>'narragansett_pier', :klass=>'CCity'},
          {:code_name=>'atlantic_ocean', :klass=>'WWaterArea'}],
       'narragansett_pier'=>
         [{:code_name=>'westerly', :klass=>'CCity'},
          {:code_name=>'apponaug', :klass=>'CCity'},
          {:code_name=>'atlantic_ocean', :klass=>'WWaterArea'}],
       'apponaug'=>
         [{:code_name=>'providence', :klass=>'CCity'},
          {:code_name=>'narragansett_pier', :klass=>'CCity'},
          {:code_name=>'atlantic_ocean', :klass=>'WWaterArea'}],
       'bristol'=>
         [{:code_name=>'providence', :klass=>'CCity'},
          {:code_name=>'fall_river', :klass=>'CCity'},
          {:code_name=>'atlantic_ocean', :klass=>'WWaterArea'}],
       'newport'=>
         [{:code_name=>'fall_river', :klass=>'CCity'},
          {:code_name=>'atlantic_ocean', :klass=>'WWaterArea'}],
       'fall_river'=>
         [{:code_name=>'providence', :klass=>'CCity'},
          {:code_name=>'taunton', :klass=>'CCity'},
          {:code_name=>'bristol', :klass=>'CCity'},
          {:code_name=>'newport', :klass=>'CCity'},
          {:code_name=>'new_bedford', :klass=>'CCity'},
          {:code_name=>'atlantic_ocean', :klass=>'WWaterArea'}],
       'new_bedford'=>
         [{:code_name=>'middleboro', :klass=>'CCity'},
          {:code_name=>'fall_river', :klass=>'CCity'},
          {:code_name=>'buzzards_bay', :klass=>'WWaterArea'}],
       'edgartown'=>[{:code_name=>'nantucket_sound', :klass=>'WWaterArea'}],
       'nantucket'=>[{:code_name=>'nantucket_sound', :klass=>'WWaterArea'}],
       'chatham'=>
         [{:code_name=>'barnstable', :klass=>'CCity'},
          {:code_name=>'provincetown', :klass=>'CCity'},
          {:code_name=>'nantucket_sound', :klass=>'WWaterArea'}],
       'plymouth'=>
         [{:code_name=>'middleboro', :klass=>'CCity'},
          {:code_name=>'greenbush', :klass=>'CCity'},
          {:code_name=>'tremont', :klass=>'CCity'},
          {:code_name=>'cape_cod_bay', :klass=>'WWaterArea'}],
       'provincetown'=>
         [{:code_name=>'barnstable', :klass=>'CCity'},
          {:code_name=>'chatham', :klass=>'CCity'},
          {:code_name=>'cape_cod_bay', :klass=>'WWaterArea'}],
       'gulf_of_maine'=>
         [{:code_name=>'newburyport', :klass=>'CCity'},
          {:code_name=>'innsmouth', :klass=>'CCity'},
          {:code_name=>'gloucester', :klass=>'CCity'},
          {:code_name=>'massachusetts_bay', :klass=>'WWaterArea'},
          {:code_name=>'nantucket_sound', :klass=>'WWaterArea'},
          {:code_name=>'cape_cod_bay', :klass=>'WWaterArea'}],
       'massachusetts_bay'=>
         [{:code_name=>'arkham', :klass=>'CCity'},
          {:code_name=>'lynn', :klass=>'CCity'},
          {:code_name=>'boston', :klass=>'CCity'},
          {:code_name=>'gulf_of_maine', :klass=>'WWaterArea'},
          {:code_name=>'cape_cod_bay', :klass=>'WWaterArea'}],
       'atlantic_ocean'=>
         [{:code_name=>'westerly', :klass=>'CCity'},
          {:code_name=>'narragansett_pier', :klass=>'CCity'},
          {:code_name=>'apponaug', :klass=>'CCity'},
          {:code_name=>'bristol', :klass=>'CCity'},
          {:code_name=>'newport', :klass=>'CCity'},
          {:code_name=>'fall_river', :klass=>'CCity'},
          {:code_name=>'buzzards_bay', :klass=>'WWaterArea'},
          {:code_name=>'nantucket_sound', :klass=>'WWaterArea'}],
       'buzzards_bay'=>
         [{:code_name=>'new_bedford', :klass=>'CCity'},
          {:code_name=>'atlantic_ocean', :klass=>'WWaterArea'},
          {:code_name=>'nantucket_sound', :klass=>'WWaterArea'}],
       'nantucket_sound'=>
         [{:code_name=>'edgartown', :klass=>'CCity'},
          {:code_name=>'nantucket', :klass=>'CCity'},
          {:code_name=>'chatham', :klass=>'CCity'},
          {:code_name=>'gulf_of_maine', :klass=>'WWaterArea'},
          {:code_name=>'atlantic_ocean', :klass=>'WWaterArea'},
          {:code_name=>'buzzards_bay', :klass=>'WWaterArea'}],
       'cape_cod_bay'=>
         [{:code_name=>'plymouth', :klass=>'CCity'},
          {:code_name=>'provincetown', :klass=>'CCity'},
          {:code_name=>'gulf_of_maine', :klass=>'WWaterArea'},
          {:code_name=>'massachusetts_bay', :klass=>'WWaterArea'}]}

      def self.find_next_dest_to_goal( current_position, goal )

        goal_name = goal.code_name

        frontier = []
        frontier << current_position.code_name
        came_from = {}
        came_from[ current_position.code_name ] = nil

        until frontier.empty?
          current_name = frontier.shift

          if current_name
            if current_name == goal_name
              break
            end

            p current_name

            DESTINATIONS[ current_name ].each do |next_location|
              unless came_from.has_key?( next_location[ 'code_name' ] )
                frontier << next_location[ 'code_name' ]
                came_from[ next_location[ 'code_name' ] ] = next_location[ 'code_name' ]
              end
            end
          end
        end

        back_token = goal_name
        next_step = nil

        pp came_from

        while back_token != current_position.code_name
          next_step = back_token
          back_token = came_from[ back_token ]
          p next_step
          break if next_step == nil
        end

        next_step = DESTINATIONS[ next_step ]
        next_step[ 'klass' ].constantize.find_by( next_step[ 'code_name' ] )
      end

      def self.create_dest_hash

        dest_hash = {}

        CCity.all.each do |c|
          dest_hash[ c.code_name ] = c.destinations.map{ |e| { code_name: e.code_name, klass: e.class.to_s } }
        end

        WWaterArea.all.each do |c|
          dest_hash[ c.code_name ] = c.destinations.map{ |e| { code_name: e.code_name, klass: e.class.to_s } }
        end

        pp dest_hash

      end
    end
  end
end
