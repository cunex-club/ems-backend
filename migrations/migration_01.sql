--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2 (Homebrew)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

ALTER TABLE IF EXISTS ONLY "public"."user_api_keys" DROP CONSTRAINT IF EXISTS "fk_users_user_api_key";
ALTER TABLE IF EXISTS ONLY "public"."projects" DROP CONSTRAINT IF EXISTS "fk_users_projects";
ALTER TABLE IF EXISTS ONLY "public"."project_members" DROP CONSTRAINT IF EXISTS "fk_users_project_members";
ALTER TABLE IF EXISTS ONLY "public"."candidates" DROP CONSTRAINT IF EXISTS "fk_question_candidates";
ALTER TABLE IF EXISTS ONLY "public"."project_members" DROP CONSTRAINT IF EXISTS "fk_projects_project_members";
ALTER TABLE IF EXISTS ONLY "public"."elections" DROP CONSTRAINT IF EXISTS "fk_project_elections";
ALTER TABLE IF EXISTS ONLY "public"."questions" DROP CONSTRAINT IF EXISTS "fk_election_questions";
ALTER TABLE IF EXISTS ONLY "public"."users" DROP CONSTRAINT IF EXISTS "users_pkey";
ALTER TABLE IF EXISTS ONLY "public"."users" DROP CONSTRAINT IF EXISTS "users_email_key";
ALTER TABLE IF EXISTS ONLY "public"."user_api_keys" DROP CONSTRAINT IF EXISTS "user_api_keys_pkey";
ALTER TABLE IF EXISTS ONLY "public"."questions" DROP CONSTRAINT IF EXISTS "questions_pkey";
ALTER TABLE IF EXISTS ONLY "public"."projects" DROP CONSTRAINT IF EXISTS "projects_pkey";
ALTER TABLE IF EXISTS ONLY "public"."project_members" DROP CONSTRAINT IF EXISTS "project_members_pkey";
ALTER TABLE IF EXISTS ONLY "public"."elections" DROP CONSTRAINT IF EXISTS "elections_pkey";
ALTER TABLE IF EXISTS ONLY "public"."candidates" DROP CONSTRAINT IF EXISTS "candidates_pkey";
ALTER TABLE IF EXISTS ONLY "logging"."api_error_logs" DROP CONSTRAINT IF EXISTS "api_error_logs_pkey";
DROP TABLE IF EXISTS "public"."users";
DROP TABLE IF EXISTS "public"."user_api_keys";
DROP TABLE IF EXISTS "public"."questions";
DROP TABLE IF EXISTS "public"."projects";
DROP TABLE IF EXISTS "public"."project_members";
DROP TABLE IF EXISTS "public"."elections";
DROP TABLE IF EXISTS "public"."candidates";
DROP TABLE IF EXISTS "logging"."api_error_logs";
DROP SCHEMA IF EXISTS "logging";
--
-- Name: logging; Type: SCHEMA; Schema: -; Owner: -
--

CREATE SCHEMA "logging";


--
-- Name: SCHEMA "public"; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON SCHEMA "public" IS 'standard public schema';


SET default_tablespace = '';

SET default_table_access_method = "heap";

--
-- Name: api_error_logs; Type: TABLE; Schema: logging; Owner: -
--

CREATE TABLE "logging"."api_error_logs" (
    "id" "uuid" NOT NULL,
    "code" integer,
    "error_type" "text",
    "detail" "text",
    "source" "text"
);


--
-- Name: candidates; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE "public"."candidates" (
    "id" "uuid" DEFAULT "gen_random_uuid"() NOT NULL,
    "question_id" "uuid" NOT NULL,
    "choice_label_th" "text" NOT NULL,
    "choice_label_en" "text" NOT NULL,
    "title" "text" NOT NULL,
    "info_line_1" "text" NOT NULL,
    "info_line_2" "text" NOT NULL,
    "info_line_3" "text" NOT NULL,
    "info_line_4" "text" NOT NULL,
    "info_line_5" "text" NOT NULL,
    "body_title_1" "text" NOT NULL,
    "body_1" "text" NOT NULL,
    "body_title_2" "text" NOT NULL,
    "body_2" "text" NOT NULL,
    "image_file" "text" NOT NULL
);


--
-- Name: elections; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE "public"."elections" (
    "id" "uuid" DEFAULT "gen_random_uuid"() NOT NULL,
    "project_id" "uuid" NOT NULL,
    "label" "text" NOT NULL,
    "name_th" "text" NOT NULL,
    "name_en" "text" NOT NULL,
    "header_th" "text" NOT NULL,
    "header_en" "text" NOT NULL,
    "detail_th" "text",
    "detail_en" "text"
);


--
-- Name: project_members; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE "public"."project_members" (
    "id" "uuid" DEFAULT "gen_random_uuid"() NOT NULL,
    "user_id" "uuid" NOT NULL,
    "project_id" "uuid" NOT NULL
);


--
-- Name: projects; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE "public"."projects" (
    "id" "uuid" DEFAULT "gen_random_uuid"() NOT NULL,
    "name" "text" NOT NULL,
    "owner_id" "uuid" NOT NULL
);


--
-- Name: questions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE "public"."questions" (
    "id" "uuid" DEFAULT "gen_random_uuid"() NOT NULL,
    "election_id" "uuid" NOT NULL,
    "question_th" "text" NOT NULL,
    "question_en" "text" NOT NULL,
    "faculty_code" "text" NOT NULL,
    "student_year_start" integer NOT NULL,
    "student_year_end" integer NOT NULL,
    "student_program" "text" NOT NULL
);


--
-- Name: user_api_keys; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE "public"."user_api_keys" (
    "id" "uuid" DEFAULT "gen_random_uuid"() NOT NULL,
    "user_id" "uuid" NOT NULL,
    "short_token" "text" NOT NULL,
    "long_token_hash" "text" NOT NULL,
    "expire_at" timestamp with time zone,
    "created_at" timestamp with time zone DEFAULT "now"()
);


--
-- Name: users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE "public"."users" (
    "id" "uuid" DEFAULT "gen_random_uuid"() NOT NULL,
    "created_at" timestamp with time zone DEFAULT "now"(),
    "email" "text" NOT NULL,
    "username" "text" DEFAULT ''::"text" NOT NULL,
    "profile" "text" DEFAULT ''::"text" NOT NULL,
    "first_name" "text" DEFAULT ''::"text" NOT NULL,
    "last_name" "text" DEFAULT ''::"text" NOT NULL,
    "is_admin" boolean DEFAULT false NOT NULL
);


--
-- Name: api_error_logs api_error_logs_pkey; Type: CONSTRAINT; Schema: logging; Owner: -
--

ALTER TABLE ONLY "logging"."api_error_logs"
    ADD CONSTRAINT "api_error_logs_pkey" PRIMARY KEY ("id");


--
-- Name: candidates candidates_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."candidates"
    ADD CONSTRAINT "candidates_pkey" PRIMARY KEY ("id");


--
-- Name: elections elections_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."elections"
    ADD CONSTRAINT "elections_pkey" PRIMARY KEY ("id");


--
-- Name: project_members project_members_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."project_members"
    ADD CONSTRAINT "project_members_pkey" PRIMARY KEY ("id");


--
-- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."projects"
    ADD CONSTRAINT "projects_pkey" PRIMARY KEY ("id");


--
-- Name: questions questions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."questions"
    ADD CONSTRAINT "questions_pkey" PRIMARY KEY ("id");


--
-- Name: user_api_keys user_api_keys_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."user_api_keys"
    ADD CONSTRAINT "user_api_keys_pkey" PRIMARY KEY ("id");


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."users"
    ADD CONSTRAINT "users_email_key" UNIQUE ("email");


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."users"
    ADD CONSTRAINT "users_pkey" PRIMARY KEY ("id");


--
-- Name: questions fk_election_questions; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."questions"
    ADD CONSTRAINT "fk_election_questions" FOREIGN KEY ("election_id") REFERENCES "public"."elections"("id");


--
-- Name: elections fk_project_elections; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."elections"
    ADD CONSTRAINT "fk_project_elections" FOREIGN KEY ("project_id") REFERENCES "public"."projects"("id");


--
-- Name: project_members fk_projects_project_members; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."project_members"
    ADD CONSTRAINT "fk_projects_project_members" FOREIGN KEY ("project_id") REFERENCES "public"."projects"("id") ON DELETE CASCADE;


--
-- Name: candidates fk_question_candidates; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."candidates"
    ADD CONSTRAINT "fk_question_candidates" FOREIGN KEY ("question_id") REFERENCES "public"."questions"("id");


--
-- Name: project_members fk_users_project_members; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."project_members"
    ADD CONSTRAINT "fk_users_project_members" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON UPDATE CASCADE;


--
-- Name: projects fk_users_projects; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."projects"
    ADD CONSTRAINT "fk_users_projects" FOREIGN KEY ("owner_id") REFERENCES "public"."users"("id") ON DELETE CASCADE;


--
-- Name: user_api_keys fk_users_user_api_key; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "public"."user_api_keys"
    ADD CONSTRAINT "fk_users_user_api_key" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id");


--
-- PostgreSQL database dump complete
--

