# This file is auto-generated from the current state of the database. Instead
# of editing this file, please use the migrations feature of Active Record to
# incrementally modify your database, and then regenerate this schema definition.
#
# Note that this schema.rb definition is the authoritative source for your
# database schema. If you need to create the application database on another
# system, you should be using db:schema:load, not running all the migrations
# from scratch. The latter is a flawed and unsustainable approach (the more migrations
# you'll amass, the slower it'll run and the greater likelihood for issues).
#
# It's strongly recommended that you check this file into your version control system.

ActiveRecord::Schema.define(version: 20170615175033) do

  # These are extensions that must be enabled in order to support this database
  enable_extension "plpgsql"

  create_table "e_event_logs", force: :cascade do |t|
    t.datetime "created_at",       null: false
    t.datetime "updated_at",       null: false
    t.integer  "g_game_board_id",  null: false
    t.integer  "turn"
    t.string   "actor_type",       null: false
    t.integer  "actor_id",         null: false
    t.string   "actor_aasm_state", null: false
    t.string   "message",          null: false
    t.index ["g_game_board_id"], name: "index_e_event_logs_on_g_game_board_id", using: :btree
    t.index ["turn"], name: "index_e_event_logs_on_turn", using: :btree
  end

  create_table "g_game_boards", force: :cascade do |t|
    t.integer  "turn",                                                      null: false
    t.datetime "created_at",                                                null: false
    t.datetime "updated_at",                                                null: false
    t.string   "aasm_state",                                                null: false
    t.integer  "players_count",                                             null: false
    t.integer  "asked_fake_cities_count"
    t.integer  "asked_fake_cities_investigator_id"
    t.integer  "nyog_sothep_invocation_position_rotation",                  null: false
    t.string   "nyog_sothep_invocation_position_code_name",                 null: false
    t.boolean  "nyog_sothep_invoked",                       default: false, null: false
  end

  create_table "i_inv_target_positions", force: :cascade do |t|
    t.integer  "g_game_board_id"
    t.integer  "memory_counter",     default: 3, null: false
    t.datetime "created_at",                     null: false
    t.datetime "updated_at",                     null: false
    t.string   "position_code_name",             null: false
    t.index ["g_game_board_id"], name: "index_i_inv_target_positions_on_g_game_board_id", using: :btree
  end

  create_table "i_investigators", force: :cascade do |t|
    t.string   "code_name",                                                 null: false
    t.integer  "san",                                                       null: false
    t.boolean  "weapon",                                    default: false, null: false
    t.boolean  "medaillon",                                 default: false, null: false
    t.boolean  "sign",                                      default: false, null: false
    t.boolean  "spell",                                     default: false, null: false
    t.datetime "created_at",                                                null: false
    t.datetime "updated_at",                                                null: false
    t.string   "gender",                          limit: 1,                 null: false
    t.integer  "event_table",                               default: 1,     null: false
    t.string   "aasm_state",                                                null: false
    t.integer  "g_game_board_id",                                           null: false
    t.integer  "token_rotation",                                            null: false
    t.string   "current_location_code_name",                                null: false
    t.string   "last_location_code_name",                                   null: false
    t.string   "ia_target_destination_code_name",                           null: false
    t.string   "forbidden_city_code_name"
    t.index ["g_game_board_id"], name: "index_i_investigators_on_g_game_board_id", using: :btree
  end

  create_table "m_monsters", force: :cascade do |t|
    t.string   "code_name",       null: false
    t.datetime "created_at",      null: false
    t.datetime "updated_at",      null: false
    t.integer  "g_game_board_id", null: false
    t.index ["g_game_board_id"], name: "index_m_monsters_on_g_game_board_id", using: :btree
  end

  create_table "p_monster_positions", force: :cascade do |t|
    t.string   "code_name",                          null: false
    t.boolean  "discovered",         default: false, null: false
    t.datetime "created_at",                         null: false
    t.datetime "updated_at",                         null: false
    t.integer  "g_game_board_id",                    null: false
    t.integer  "token_rotation",                     null: false
    t.string   "location_code_name",                 null: false
    t.index ["g_game_board_id"], name: "index_p_monster_positions_on_g_game_board_id", using: :btree
  end

  create_table "p_monsters", force: :cascade do |t|
    t.string   "code_name",       null: false
    t.datetime "created_at",      null: false
    t.datetime "updated_at",      null: false
    t.integer  "g_game_board_id", null: false
    t.index ["g_game_board_id"], name: "index_p_monsters_on_g_game_board_id", using: :btree
  end

  create_table "p_professors", force: :cascade do |t|
    t.integer  "hp",                         null: false
    t.datetime "created_at",                 null: false
    t.datetime "updated_at",                 null: false
    t.integer  "g_game_board_id",            null: false
    t.integer  "token_rotation",             null: false
    t.string   "current_location_code_name", null: false
    t.index ["g_game_board_id"], name: "index_p_professors_on_g_game_board_id", using: :btree
  end

  add_foreign_key "e_event_logs", "g_game_boards"
  add_foreign_key "i_investigators", "g_game_boards"
  add_foreign_key "m_monsters", "g_game_boards"
  add_foreign_key "p_monster_positions", "g_game_boards"
  add_foreign_key "p_monsters", "g_game_boards"
  add_foreign_key "p_professors", "g_game_boards"
end
