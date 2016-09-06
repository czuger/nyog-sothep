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

ActiveRecord::Schema.define(version: 20160906175206) do

  # These are extensions that must be enabled in order to support this database
  enable_extension "plpgsql"

  create_table "c_cities", force: :cascade do |t|
    t.string   "code_name",       null: false
    t.integer  "x",               null: false
    t.integer  "y",               null: false
    t.datetime "created_at",      null: false
    t.datetime "updated_at",      null: false
    t.boolean  "port"
    t.integer  "w_water_area_id"
    t.index ["code_name"], name: "index_c_cities_on_code_name", unique: true, using: :btree
    t.index ["w_water_area_id"], name: "index_c_cities_on_w_water_area_id", using: :btree
  end

  create_table "r_roads", force: :cascade do |t|
    t.integer  "src_city_id",  null: false
    t.integer  "dest_city_id", null: false
    t.boolean  "border"
    t.datetime "created_at",   null: false
    t.datetime "updated_at",   null: false
    t.index ["dest_city_id"], name: "index_r_roads_on_dest_city_id", using: :btree
    t.index ["src_city_id", "dest_city_id"], name: "index_r_roads_on_src_city_id_and_dest_city_id", unique: true, using: :btree
    t.index ["src_city_id"], name: "index_r_roads_on_src_city_id", using: :btree
  end

  create_table "w_water_area_connections", force: :cascade do |t|
    t.integer  "src_w_water_area_id"
    t.integer  "dest_w_water_area_id"
    t.datetime "created_at",           null: false
    t.datetime "updated_at",           null: false
    t.index ["dest_w_water_area_id"], name: "index_w_water_area_connections_on_dest_w_water_area_id", using: :btree
    t.index ["src_w_water_area_id", "dest_w_water_area_id"], name: "w_water_area_connections_id", unique: true, using: :btree
    t.index ["src_w_water_area_id"], name: "index_w_water_area_connections_on_src_w_water_area_id", using: :btree
  end

  create_table "w_water_areas", force: :cascade do |t|
    t.string   "code_name"
    t.integer  "x"
    t.integer  "y"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
  end

  add_foreign_key "c_cities", "w_water_areas"
  add_foreign_key "r_roads", "c_cities", column: "dest_city_id"
  add_foreign_key "r_roads", "c_cities", column: "src_city_id"
  add_foreign_key "w_water_area_connections", "w_water_areas", column: "dest_w_water_area_id"
  add_foreign_key "w_water_area_connections", "w_water_areas", column: "src_w_water_area_id"
end
