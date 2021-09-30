pub mod insert{
    use mysql::*;
    use mysql::prelude::*;

    use crate::structs::database::database;
    use crate::structs::auth::auth::Auth;
    use crate::structs::moderation::moderation::ModerationRecord;
    use crate::structs::user::user::User;
    use crate::auth::user::create;
    use crate::structs::requests::post::{MakeCat, MakeComment, MakePost};
    use crate::utils::ip_tools::ip_tools;

    use std::time::SystemTime;

    const URL:&str = "mysql://server:serverpass@10.16.40.202:3306/forum";

    pub fn add_user(newuser: database::NewUser) -> Result<String>{//idk what this should be returning realistically
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let auth: Auth = create::create_auth(newuser.clone());
        let modrec: ModerationRecord = create::create_mod_rec();
        let user: User = create::create_user(newuser.clone(), auth.auth_id, modrec.moderation_id);
        let stmt = conn.prep("INSERT INTO users (username,locale,auth_id, moderation_id) VALUES(:uname,:locale,:auth_id,:mod_id)")?;
        conn.exec_drop(&stmt, params!{
            "uname" => user.username.clone(),
            "locale" => user.locale.clone(),
            "auth_id" => user.auth_id,
            "mod_id" => user.moderation_id,
        },).unwrap();

        Ok("user added".to_owned())
    }
    pub fn add_auth(new_auth: &Auth) -> Result<u64>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let stmt = conn.prep("INSERT INTO auth (creation_ip, last_ip, email, pwhash, uname, last_change, created) VALUES(:creation_ip, :last_ip, :email, :pwhash, :uname, :last_change, :created)")?;
        conn.exec_drop(&stmt, params!{
            "creation_ip" => ip_tools::strip_port(new_auth.creation_ip.clone()),
            "last_ip" => ip_tools::strip_port(new_auth.last_ip.clone()),
            "email" => new_auth.email.clone(),
            "pwhash" => new_auth.hash.clone(),
            "uname" => new_auth.uname.clone(),
            "last_change" => new_auth.last_change,
            "created" => new_auth.created,
        },).unwrap();

        Ok(conn.last_insert_id())
    }
    pub fn add_modrec() -> Result<u64>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let stmt = conn.prep("INSERT INTO user_moderation (infraction_counter) VALUES(:infraction_counter)")?;
        conn.exec_drop(&stmt, params!{
            "infraction_counter" => "0",
        },).unwrap();
        //do something with this

        Ok(conn.last_insert_id())
    }

    //insert a new token into the database for user with aid u64
    pub fn add_token(token: String, aid: u64, ip: String) -> Result<bool>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        log::info!("adding token {} to auth for aid {}", token, aid);
        let stmt = conn.prep("UPDATE auth SET active_token = :token, token_ip = :ip, token_date = :date WHERE auth_id = :aid")?;
        conn.exec_drop(&stmt, params!{
            "token" => token,
            "aid" => aid,
            "ip" => ip_tools::strip_port(ip),
            "date" => date,
        },).unwrap();
        Ok(true)
    }
    pub fn create_cat(cat: MakeCat) -> Result<u64>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        log::info!("adding cat {} to categories for uid {}", cat.name.clone(), cat.user);
        let stmt = conn.prep("INSERT INTO categories (creator_id, cat_name, cat_desc, creation_date) VALUES(:uid, :name, :desc, :date)")?;
        conn.exec_drop(&stmt, params!{
            "date" => date,
            "desc" => cat.desc.clone(),
            "uid" => cat.user,
            "name" => cat.name.clone(),
        },).unwrap();
        let cat_id:u64 = conn.last_insert_id();
        Ok(cat_id)
    }
    pub fn create_post(post: MakePost) -> Result<u64>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        log::info!("adding post {} to category {} for uid {}", post.name.clone(), post.cat, post.user);
        let stmt = conn.prep("INSERT INTO posts (creator_id, cat_id, content, name, creation_date) VALUES(:uid, :cat_id, :content, :name, :date)")?;
        conn.exec_drop(&stmt, params!{
            "cat_id" => post.cat,
            "date" => date,
            "content" => post.contents.clone(),
            "uid" => post.user,
            "name" => post.name.clone(),
        },).unwrap();
        let post_id:u64 = conn.last_insert_id();
        Ok(post_id)
    }
    pub fn create_comment(comment: MakeComment) -> Result<u64>{
        let opts = Opts::from_url(URL).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();
        let date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        log::info!("adding comment to post {} for uid {}", comment.post, comment.user);
        let stmt = conn.prep("INSERT INTO comments (creator_id, post_id, parent_id, content, creation_date) VALUES(:uid, :post_id, :parent_id, :content, :date)")?;
        conn.exec_drop(&stmt, params!{
            "creator_id" => comment.user,
            "date" => date,
            "content" => comment.contents.clone(),
            "uid" => comment.user,
            "parent_id" => comment.parent,
        },).unwrap();
        let comment_id:u64 = conn.last_insert_id();
        Ok(comment_id)
    }
}