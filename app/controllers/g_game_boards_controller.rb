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
  end

  # GET /g_game_boards/1/edit
  def edit
  end

  # POST /g_game_boards
  # POST /g_game_boards.json
  def create
    @g_game_board = GGameBoard.new(g_game_board_params)

    @g_game_board.players_count = 1
    @g_game_board.ia_side = 'inv'

    @g_game_board.prof_security_code = rand.to_s

    @g_game_board.turn = 0

    respond_to do |format|
      if @g_game_board.save

        @g_game_board.next_turn

        GameCore::GGameBoardCreation.populate_game_board( @g_game_board )

        format.html { redirect_to @g_game_board, notice: 'G game board was successfully created.' }
        format.json { render :show, status: :created, location: @g_game_board }
      else
        format.html { render :new }
        format.json { render json: @g_game_board.errors, status: :unprocessable_entity }
      end
    end
  end

  # PATCH/PUT /g_game_boards/1
  # PATCH/PUT /g_game_boards/1.json
  def update
    respond_to do |format|
      if @g_game_board.update(g_game_board_params)
        format.html { redirect_to @g_game_board, notice: 'G game board was successfully updated.' }
        format.json { render :show, status: :ok, location: @g_game_board }
      else
        format.html { render :edit }
        format.json { render json: @g_game_board.errors, status: :unprocessable_entity }
      end
    end
  end

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
