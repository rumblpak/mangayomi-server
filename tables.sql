-- -----------------------------------------------------
-- Table `accounts`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `accounts` ;

CREATE TABLE IF NOT EXISTS `accounts` (
  `id` INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
  `email` VARCHAR(512) NOT NULL,
  `password` VARCHAR(1024) NOT NULL,
  `salt` VARCHAR(1024) NOT NULL)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `item_types`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `item_types` ;

CREATE TABLE IF NOT EXISTS `item_types` (
  `index` INT NOT NULL,
  `name` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`index`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `status`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `status` ;

CREATE TABLE IF NOT EXISTS `status` (
  `index` INT NOT NULL,
  `name` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`index`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `manga`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `manga` ;

CREATE TABLE IF NOT EXISTS `manga` (
  `id` INT NOT NULL,
  `name` VARCHAR(1024) NOT NULL,
  `link` VARCHAR(1024) NOT NULL,
  `image_url` VARCHAR(1024) NOT NULL,
  `description` VARCHAR(4096) NULL,
  `author` VARCHAR(512) NULL,
  `artist` VARCHAR(512) NULL,
  `status_index` INT NOT NULL,
  `favorite` TINYINT NOT NULL,
  `source` VARCHAR(512) NOT NULL,
  `lang` VARCHAR(45) NOT NULL,
  `date_added` INT NOT NULL,
  `last_update` INT NULL,
  `last_read` INT NULL,
  `is_local_archive` TINYINT NULL,
  `custom_cover_image` BLOB NULL,
  `custom_cover_from_tracker` VARCHAR(1024) NULL,
  `item_type` INT NOT NULL,
  `user` INT NOT NULL,
  `genres` VARCHAR(1024) NULL,
  `updated_at` INT NOT NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_manga_item_types_idx` (`item_type` ASC) VISIBLE,
  INDEX `fk_manga_users1_idx` (`user` ASC) VISIBLE,
  INDEX `fk_manga_status1_idx` (`status_index` ASC) VISIBLE,
  CONSTRAINT `fk_manga_item_types`
    FOREIGN KEY (`item_type`)
    REFERENCES `item_types` (`index`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_manga_users1`
    FOREIGN KEY (`user`)
    REFERENCES `accounts` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_manga_status1`
    FOREIGN KEY (`status_index`)
    REFERENCES `status` (`index`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `categories`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `categories` ;

CREATE TABLE IF NOT EXISTS `categories` (
  `id` INT NOT NULL,
  `name` VARCHAR(512) NOT NULL,
  `for_item_type` INT NOT NULL,
  `pos` INT NULL,
  `hide` TINYINT NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_categories_item_types1_idx` (`for_item_type` ASC) VISIBLE,
  CONSTRAINT `fk_categories_item_types1`
    FOREIGN KEY (`for_item_type`)
    REFERENCES `item_types` (`index`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `chapters`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `chapters` ;

CREATE TABLE IF NOT EXISTS `chapters` (
  `id` INT NOT NULL,
  `name` VARCHAR(1024) NOT NULL,
  `url` VARCHAR(1024) NOT NULL,
  `date_upload` VARCHAR(512) NOT NULL,
  `scanlator` VARCHAR(512) NULL,
  `is_bookmarked` TINYINT NOT NULL,
  `is_read` TINYINT NOT NULL,
  `last_page_read` VARCHAR(512) NOT NULL,
  `archive_path` VARCHAR(1024) NULL,
  `manga` INT NOT NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_chapters_manga1_idx` (`manga` ASC) VISIBLE,
  CONSTRAINT `fk_chapters_manga1`
    FOREIGN KEY (`manga`)
    REFERENCES `manga` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `histories`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `histories` ;

CREATE TABLE IF NOT EXISTS `histories` (
  `id` INT NOT NULL,
  `date` VARCHAR(512) NOT NULL,
  `chapter` INT NOT NULL,
  `item_type` INT NOT NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_histories_chapters1_idx` (`chapter` ASC) VISIBLE,
  INDEX `fk_histories_item_types1_idx` (`item_type` ASC) VISIBLE,
  CONSTRAINT `fk_histories_chapters1`
    FOREIGN KEY (`chapter`)
    REFERENCES `chapters` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_histories_item_types1`
    FOREIGN KEY (`item_type`)
    REFERENCES `item_types` (`index`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `sources`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `sources` ;

CREATE TABLE IF NOT EXISTS `sources` (
  `id` INT NOT NULL,
  `api_url` VARCHAR(1024) NULL,
  `app_min_ver_req` VARCHAR(512) NULL,
  `base_url` VARCHAR(1024) NOT NULL,
  `date_format` VARCHAR(512) NULL,
  `date_format_locale` VARCHAR(45) NULL,
  `has_cloudflare` TINYINT NULL,
  `headers` VARCHAR(1024) NULL,
  `icon_url` VARCHAR(1024) NULL,
  `is_active` TINYINT NOT NULL,
  `is_added` TINYINT NULL,
  `is_full_data` TINYINT NULL,
  `is_manga` TINYINT NULL,
  `item_type` INT NOT NULL,
  `is_nsfw` TINYINT NULL,
  `is_pinned` TINYINT NULL,
  `lang` VARCHAR(45) NOT NULL,
  `last_used` TINYINT NULL,
  `name` VARCHAR(512) NOT NULL,
  `source_code_url` VARCHAR(1024) NOT NULL,
  `type_source` VARCHAR(512) NULL,
  `version` VARCHAR(512) NOT NULL,
  `version_last` VARCHAR(512) NULL,
  `additional_params` VARCHAR(1024) NULL,
  `source_code_language` VARCHAR(45) NULL,
  `is_obsolete` TINYINT NULL,
  `is_local` TINYINT NULL,
  `user` INT NOT NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_sources_item_types1_idx` (`item_type` ASC) VISIBLE,
  INDEX `fk_sources_users1_idx` (`user` ASC) VISIBLE,
  CONSTRAINT `fk_sources_item_types1`
    FOREIGN KEY (`item_type`)
    REFERENCES `item_types` (`index`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_sources_users1`
    FOREIGN KEY (`user`)
    REFERENCES `accounts` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `track_status`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `track_status` ;

CREATE TABLE IF NOT EXISTS `track_status` (
  `index` INT NOT NULL,
  `name` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`index`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `tracks`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `tracks` ;

CREATE TABLE IF NOT EXISTS `tracks` (
  `id` INT NOT NULL,
  `library_id` INT NOT NULL,
  `media_id` INT NOT NULL,
  `manga_id` INT NOT NULL,
  `score` INT NULL,
  `started_reading_date` INT NULL,
  `finished_reading_date` INT NULL,
  `last_chapter_read` INT NULL,
  `track_status_index` INT NOT NULL,
  `sync_id` INT NOT NULL,
  `title` VARCHAR(45) NOT NULL,
  `total_chapter` INT NULL,
  `tracking_url` VARCHAR(45) NOT NULL,
  `is_manga` TINYINT NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_tracks_manga1_idx` (`manga_id` ASC) VISIBLE,
  INDEX `fk_tracks_track_status1_idx` (`track_status_index` ASC) VISIBLE,
  CONSTRAINT `fk_tracks_manga1`
    FOREIGN KEY (`manga_id`)
    REFERENCES `manga` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_tracks_track_status1`
    FOREIGN KEY (`track_status_index`)
    REFERENCES `track_status` (`index`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `updates`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `updates` ;

CREATE TABLE IF NOT EXISTS `updates` (
  `id` INT NOT NULL,
  `chapter_name` VARCHAR(1024) NOT NULL,
  `date` VARCHAR(512) NOT NULL,
  `chapter` INT NOT NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_updates_chapters1_idx` (`chapter` ASC) VISIBLE,
  CONSTRAINT `fk_updates_chapters1`
    FOREIGN KEY (`chapter`)
    REFERENCES `chapters` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;
