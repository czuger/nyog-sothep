class GGameBoardsController < ApplicationController
  before_action :set_g_game_board, only: [:show, :edit, :update, :destroy]

  # GET /g_game_boards
  # GET /g_game_boards.json
  def index
    @g_game_boards = GGameBoard.all
  end

  # GET /g_game_boards/1
  # GET /g_game_boards/1.json
  def show
  end

  # GET /g_game_boards/new
  def new
    @g_game_board = GGameBoard.new
    @cities = CCity.all
  end

  # POST /g_game_boards
  # POST /g_game_boards.json
  def create
    @g_game_board = GGameBoard.new(g_game_board_params)

    @g_game_board.players_count = 1
    @g_game_board.turn = 1

    @prof_start_position = params[ 'prof_position' ].empty? ? CCity.all.sample : CCity.find( params[ 'prof_position' ] )
    assert( @prof_start_position, "Prof start position is nil : #{params[ 'prof_position' ]}" )

    @nyog_start_position = params[ 'nyog_sothep_position' ].empty? ? CCity.all.sample : CCity.find( params[ 'nyog_sothep_position' ] )
    assert( @nyog_start_position, "Nyog start position is nil : #{params[ 'nyog_sothep_position' ]}" )

    @g_game_board.nyog_sothep_invocation_position = @nyog_start_position
    @g_game_board.nyog_sothep_invocation_position_rotation = rand( -15 .. 15 )

    respond_to do |format|
      if @g_game_board.save

        GameCore::GGameBoardCreation.populate_game_board( @g_game_board, @prof_start_position )

        format.html { redirect_to g_game_board_play_url( @g_game_board ) }
        format.json { render :show, status: :created, location: @g_game_board }
      else
        format.html { render :new }
        format.json { render json: @g_game_board.errors, status: :unprocessable_entity }
      end
    end
  end

  # PATCH/PUT /g_game_boards/1
  # PATCH/PUT /g_game_boards/1.json


  # DELETE /g_game_boards/1
  # DELETE /g_game_boards/1.json
  def destroy
    @g_game_board.destroy
    respond_to do |format|
      format.html { redirect_to g_game_boards_url, notice: 'G game board was successfully destroyed.' }
      format.json { head :no_content }
    end
  end

  private
    # Use callbacks to share common setup or constraints between actions.
    def set_g_game_board
      @g_game_board = GGameBoard.find(params[:id])
    end

    # Never trust parameters from the scary internet, only allow the white list through.
    def g_game_board_params
      # params.require(:g_game_board)    # .permit(:players_count, :ia_side)
    end

end
