module GameLogicM
  class Event

    def self.e1( investigator )
      EEventLog.create( event: 'se perd dans les rues et est retardée' )
      investigator.update_attribute( :delayed, true )
    end

    # Implement $ gain on 2
    def self.e3( investigator )
      EEventLog.create( event: "rencontre un vieillard qui lui parle longuement des créatures de l'au dela. Gagne un médaillon protecteur et perd 2 points de SAN" )
      investigator.decrement( :san, 2 )
      investigator.update_attribute( :medaillon, true )
    end

    def self.method_missing( _, _ )
      EEventLog.create( event: 'event non implementé' )
    end

  end
end

