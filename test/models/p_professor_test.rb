require 'test_helper'

class PProfessorTest < ActiveSupport::TestCase

  def test_p_professor_creation_don_t_fail
    create :p_professor, g_game_board: create( :g_game_board )
  end

end
