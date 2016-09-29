class GameCore::Dices

  def self.method_missing( method_name, amount = 1 )

    # puts method_name.inspect
    dice_amount_match = method_name.to_s.match( /d(\d+)/ )
    raise "Method mising : #{method_name}" unless dice_amount_match

    # puts dice_amount_match.inspect

    dice_amount = dice_amount_match[1].to_i

    (1..amount).inject(0) { |sum, _| sum + Kernel.rand( 1..dice_amount ) }
  end

end