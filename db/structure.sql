--
-- PostgreSQL database dump
--

SET statement_timeout = 0;
SET lock_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;

--
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


SET search_path = public, pg_catalog;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: ar_internal_metadata; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE ar_internal_metadata (
    key character varying NOT NULL,
    value character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


--
-- Name: c_cities; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE c_cities (
    id integer NOT NULL,
    code_name character varying NOT NULL,
    x integer NOT NULL,
    y integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    port boolean,
    w_water_area_id integer
);


--
-- Name: c_cities_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE c_cities_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: c_cities_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE c_cities_id_seq OWNED BY c_cities.id;


--
-- Name: e_event_logs; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE e_event_logs (
    id integer NOT NULL,
    event character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    logset integer NOT NULL,
    g_game_board_id integer NOT NULL
);


--
-- Name: e_event_logs_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE e_event_logs_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: e_event_logs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE e_event_logs_id_seq OWNED BY e_event_logs.id;


--
-- Name: e_event_logs_logset; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE e_event_logs_logset
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: g_game_boards; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE g_game_boards (
    id integer NOT NULL,
    turn integer,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    aasm_state character varying NOT NULL
);


--
-- Name: g_game_boards_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE g_game_boards_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: g_game_boards_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE g_game_boards_id_seq OWNED BY g_game_boards.id;


--
-- Name: i_investigators; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE i_investigators (
    id integer NOT NULL,
    code_name character varying NOT NULL,
    san integer NOT NULL,
    weapon boolean DEFAULT false NOT NULL,
    medaillon boolean DEFAULT false NOT NULL,
    sign boolean DEFAULT false NOT NULL,
    spell boolean DEFAULT false NOT NULL,
    current boolean DEFAULT false NOT NULL,
    current_location_type character varying NOT NULL,
    current_location_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    gender character varying(1) NOT NULL,
    event_table integer DEFAULT 1 NOT NULL,
    aasm_state character varying NOT NULL,
    g_game_board_id integer NOT NULL,
    last_location_type character varying NOT NULL,
    last_location_id integer NOT NULL
);


--
-- Name: i_investigators_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE i_investigators_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: i_investigators_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE i_investigators_id_seq OWNED BY i_investigators.id;


--
-- Name: m_monsters; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE m_monsters (
    id integer NOT NULL,
    code_name character varying NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    g_game_board_id integer NOT NULL
);


--
-- Name: m_monsters_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE m_monsters_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: m_monsters_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE m_monsters_id_seq OWNED BY m_monsters.id;


--
-- Name: p_monster_positions; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE p_monster_positions (
    id integer NOT NULL,
    location_type character varying NOT NULL,
    location_id integer NOT NULL,
    code_name character varying NOT NULL,
    discovered boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    g_game_board_id integer NOT NULL
);


--
-- Name: p_monster_positions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE p_monster_positions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: p_monster_positions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE p_monster_positions_id_seq OWNED BY p_monster_positions.id;


--
-- Name: p_monsters; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE p_monsters (
    id integer NOT NULL,
    code_name character varying NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    g_game_board_id integer NOT NULL
);


--
-- Name: p_monsters_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE p_monsters_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: p_monsters_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE p_monsters_id_seq OWNED BY p_monsters.id;


--
-- Name: p_prof_positions; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE p_prof_positions (
    id integer NOT NULL,
    position_type character varying NOT NULL,
    position_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    g_game_board_id integer NOT NULL
);


--
-- Name: p_prof_positions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE p_prof_positions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: p_prof_positions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE p_prof_positions_id_seq OWNED BY p_prof_positions.id;


--
-- Name: p_professors; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE p_professors (
    id integer NOT NULL,
    hp integer NOT NULL,
    current_location_type character varying NOT NULL,
    current_location_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    g_game_board_id integer NOT NULL,
    spotted boolean DEFAULT false NOT NULL
);


--
-- Name: p_professors_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE p_professors_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: p_professors_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE p_professors_id_seq OWNED BY p_professors.id;


--
-- Name: r_roads; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE r_roads (
    id integer NOT NULL,
    src_city_id integer NOT NULL,
    dest_city_id integer NOT NULL,
    border boolean,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


--
-- Name: r_roads_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE r_roads_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: r_roads_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE r_roads_id_seq OWNED BY r_roads.id;


--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE schema_migrations (
    version character varying NOT NULL
);


--
-- Name: w_water_area_connections; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE w_water_area_connections (
    id integer NOT NULL,
    src_w_water_area_id integer,
    dest_w_water_area_id integer,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


--
-- Name: w_water_area_connections_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE w_water_area_connections_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: w_water_area_connections_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE w_water_area_connections_id_seq OWNED BY w_water_area_connections.id;


--
-- Name: w_water_areas; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE w_water_areas (
    id integer NOT NULL,
    code_name character varying,
    x integer,
    y integer,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


--
-- Name: w_water_areas_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE w_water_areas_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: w_water_areas_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE w_water_areas_id_seq OWNED BY w_water_areas.id;


--
-- Name: id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY c_cities ALTER COLUMN id SET DEFAULT nextval('c_cities_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY e_event_logs ALTER COLUMN id SET DEFAULT nextval('e_event_logs_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY g_game_boards ALTER COLUMN id SET DEFAULT nextval('g_game_boards_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY i_investigators ALTER COLUMN id SET DEFAULT nextval('i_investigators_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY m_monsters ALTER COLUMN id SET DEFAULT nextval('m_monsters_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY p_monster_positions ALTER COLUMN id SET DEFAULT nextval('p_monster_positions_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY p_monsters ALTER COLUMN id SET DEFAULT nextval('p_monsters_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY p_prof_positions ALTER COLUMN id SET DEFAULT nextval('p_prof_positions_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY p_professors ALTER COLUMN id SET DEFAULT nextval('p_professors_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY r_roads ALTER COLUMN id SET DEFAULT nextval('r_roads_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY w_water_area_connections ALTER COLUMN id SET DEFAULT nextval('w_water_area_connections_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY w_water_areas ALTER COLUMN id SET DEFAULT nextval('w_water_areas_id_seq'::regclass);


--
-- Name: ar_internal_metadata_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY ar_internal_metadata
    ADD CONSTRAINT ar_internal_metadata_pkey PRIMARY KEY (key);


--
-- Name: c_cities_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY c_cities
    ADD CONSTRAINT c_cities_pkey PRIMARY KEY (id);


--
-- Name: e_event_logs_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY e_event_logs
    ADD CONSTRAINT e_event_logs_pkey PRIMARY KEY (id);


--
-- Name: g_game_boards_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY g_game_boards
    ADD CONSTRAINT g_game_boards_pkey PRIMARY KEY (id);


--
-- Name: i_investigators_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY i_investigators
    ADD CONSTRAINT i_investigators_pkey PRIMARY KEY (id);


--
-- Name: m_monsters_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY m_monsters
    ADD CONSTRAINT m_monsters_pkey PRIMARY KEY (id);


--
-- Name: p_monster_positions_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY p_monster_positions
    ADD CONSTRAINT p_monster_positions_pkey PRIMARY KEY (id);


--
-- Name: p_monsters_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY p_monsters
    ADD CONSTRAINT p_monsters_pkey PRIMARY KEY (id);


--
-- Name: p_prof_positions_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY p_prof_positions
    ADD CONSTRAINT p_prof_positions_pkey PRIMARY KEY (id);


--
-- Name: p_professors_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY p_professors
    ADD CONSTRAINT p_professors_pkey PRIMARY KEY (id);


--
-- Name: r_roads_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY r_roads
    ADD CONSTRAINT r_roads_pkey PRIMARY KEY (id);


--
-- Name: schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: w_water_area_connections_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY w_water_area_connections
    ADD CONSTRAINT w_water_area_connections_pkey PRIMARY KEY (id);


--
-- Name: w_water_areas_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY w_water_areas
    ADD CONSTRAINT w_water_areas_pkey PRIMARY KEY (id);


--
-- Name: index_c_cities_on_code_name; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE UNIQUE INDEX index_c_cities_on_code_name ON c_cities USING btree (code_name);


--
-- Name: index_c_cities_on_w_water_area_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_c_cities_on_w_water_area_id ON c_cities USING btree (w_water_area_id);


--
-- Name: index_e_event_logs_on_g_game_board_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_e_event_logs_on_g_game_board_id ON e_event_logs USING btree (g_game_board_id);


--
-- Name: index_i_investigators_on_current; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_i_investigators_on_current ON i_investigators USING btree (current);


--
-- Name: index_i_investigators_on_g_game_board_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_i_investigators_on_g_game_board_id ON i_investigators USING btree (g_game_board_id);


--
-- Name: index_m_monsters_on_g_game_board_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_m_monsters_on_g_game_board_id ON m_monsters USING btree (g_game_board_id);


--
-- Name: index_p_monster_positions_on_g_game_board_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_p_monster_positions_on_g_game_board_id ON p_monster_positions USING btree (g_game_board_id);


--
-- Name: index_p_monster_positions_on_location_type_and_location_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_p_monster_positions_on_location_type_and_location_id ON p_monster_positions USING btree (location_type, location_id);


--
-- Name: index_p_monsters_on_g_game_board_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_p_monsters_on_g_game_board_id ON p_monsters USING btree (g_game_board_id);


--
-- Name: index_p_prof_positions_on_g_game_board_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_p_prof_positions_on_g_game_board_id ON p_prof_positions USING btree (g_game_board_id);


--
-- Name: index_p_prof_positions_on_position_type_and_position_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_p_prof_positions_on_position_type_and_position_id ON p_prof_positions USING btree (position_type, position_id);


--
-- Name: index_p_professors_on_g_game_board_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_p_professors_on_g_game_board_id ON p_professors USING btree (g_game_board_id);


--
-- Name: index_r_roads_on_dest_city_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_r_roads_on_dest_city_id ON r_roads USING btree (dest_city_id);


--
-- Name: index_r_roads_on_src_city_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_r_roads_on_src_city_id ON r_roads USING btree (src_city_id);


--
-- Name: index_r_roads_on_src_city_id_and_dest_city_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE UNIQUE INDEX index_r_roads_on_src_city_id_and_dest_city_id ON r_roads USING btree (src_city_id, dest_city_id);


--
-- Name: index_w_water_area_connections_on_dest_w_water_area_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_w_water_area_connections_on_dest_w_water_area_id ON w_water_area_connections USING btree (dest_w_water_area_id);


--
-- Name: index_w_water_area_connections_on_src_w_water_area_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX index_w_water_area_connections_on_src_w_water_area_id ON w_water_area_connections USING btree (src_w_water_area_id);


--
-- Name: w_water_area_connections_id; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE UNIQUE INDEX w_water_area_connections_id ON w_water_area_connections USING btree (src_w_water_area_id, dest_w_water_area_id);


--
-- Name: fk_rails_04e46903cb; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY i_investigators
    ADD CONSTRAINT fk_rails_04e46903cb FOREIGN KEY (g_game_board_id) REFERENCES g_game_boards(id);


--
-- Name: fk_rails_37f3281d86; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY p_monster_positions
    ADD CONSTRAINT fk_rails_37f3281d86 FOREIGN KEY (g_game_board_id) REFERENCES g_game_boards(id);


--
-- Name: fk_rails_3b65d38fe5; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY w_water_area_connections
    ADD CONSTRAINT fk_rails_3b65d38fe5 FOREIGN KEY (dest_w_water_area_id) REFERENCES w_water_areas(id);


--
-- Name: fk_rails_3e4a7cb933; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY e_event_logs
    ADD CONSTRAINT fk_rails_3e4a7cb933 FOREIGN KEY (g_game_board_id) REFERENCES g_game_boards(id);


--
-- Name: fk_rails_3e59cc214e; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY r_roads
    ADD CONSTRAINT fk_rails_3e59cc214e FOREIGN KEY (src_city_id) REFERENCES c_cities(id);


--
-- Name: fk_rails_757d775233; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY p_professors
    ADD CONSTRAINT fk_rails_757d775233 FOREIGN KEY (g_game_board_id) REFERENCES g_game_boards(id);


--
-- Name: fk_rails_87ea7b5413; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY r_roads
    ADD CONSTRAINT fk_rails_87ea7b5413 FOREIGN KEY (dest_city_id) REFERENCES c_cities(id);


--
-- Name: fk_rails_8d61e104fb; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY w_water_area_connections
    ADD CONSTRAINT fk_rails_8d61e104fb FOREIGN KEY (src_w_water_area_id) REFERENCES w_water_areas(id);


--
-- Name: fk_rails_935983936c; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY p_monsters
    ADD CONSTRAINT fk_rails_935983936c FOREIGN KEY (g_game_board_id) REFERENCES g_game_boards(id);


--
-- Name: fk_rails_94d1e71349; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY m_monsters
    ADD CONSTRAINT fk_rails_94d1e71349 FOREIGN KEY (g_game_board_id) REFERENCES g_game_boards(id);


--
-- Name: fk_rails_dc82bf0c7f; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY p_prof_positions
    ADD CONSTRAINT fk_rails_dc82bf0c7f FOREIGN KEY (g_game_board_id) REFERENCES g_game_boards(id);


--
-- Name: fk_rails_f7bd3a48b9; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY c_cities
    ADD CONSTRAINT fk_rails_f7bd3a48b9 FOREIGN KEY (w_water_area_id) REFERENCES w_water_areas(id);


--
-- PostgreSQL database dump complete
--

SET search_path TO "$user",public;

INSERT INTO schema_migrations (version) VALUES ('20160906133500'), ('20160906133659'), ('20160906161013'), ('20160906163112'), ('20160906163944'), ('20160906175206'), ('20160907124113'), ('20160908143137'), ('20160909130136'), ('20160909131927'), ('20160910151650'), ('20160910163125'), ('20160911061732'), ('20160911093716'), ('20160911152718'), ('20160911161508'), ('20160911162117'), ('20160912123215'), ('20160912124640'), ('20160913131344'), ('20160914131319');


