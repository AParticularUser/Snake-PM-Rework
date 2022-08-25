use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::{
            Hash40,
            Vector2f//,
            // Vector3f
        },
        app::{
            sv_animcmd::frame,
            sv_animcmd::wait,
            lua_bind::*,
            *
        },
        lib::{
            lua_const::*,
            *
        }
    },
    smashline::*,
    smash_script::{
        macros::*,
        *
    }
};

static mut SNAKE_FLAG_SPECIAL_LW_SELF_STICK : [bool; 8] = [false; 8];
static mut SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE : [bool; 8] = [false; 8];
static mut SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED : [bool; 8] = [false; 8];
static mut SNAKE_INT_ATTACK_S4_COMBO_COUNT : [i32; 8] = [0; 8];
static mut SNAKE_FLAG_APPEAL_LW_C4_EXLPODE : [bool; 8] = [false; 8];
static mut SNAKE_FLAG_APPEAL_LW_GRENADE_WAIT_COUNT : [i32; 8] = [0; 8];
// static mut SNAKE_C4_FLAG_IS_SHOWTIME : [bool; 8] = [false; 8];
static mut SNAKE_FLAG_CATCH_WAIT_IS_WALK : [bool; 8] = [false; 8];
static SNAKE_APPEAL_LW_GRENADE_WAIT_MAX : i32 = 30;

//implimented function for checking if an article is "constrained" to snake
extern "C" {
    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Snake21is_constraint_articleERNS_7FighterEiNS_22ArticleOperationTargetE"]
    pub fn is_constraint_article(
        arg1: *mut smash::app::Fighter,
        arg2: i32,
        arg3: smash::app::ArticleOperationTarget,
    ) -> bool;
}

#[fighter_frame( agent = FIGHTER_KIND_SNAKE )]
fn snake_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4 {
            if SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] == false {
                if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SMASH) {
                    SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = true;
                }
            }
            if SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id]
            && SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] {
                SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
                SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = false;
                if SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] == 0 {
                    SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] = 1;
                    ControlModule::reset_trigger(fighter.module_accessor);
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);
                }else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s3"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
    }
}

#[weapon_frame( agent = WEAPON_KIND_SNAKE_C4 )]
fn snake_c4_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        if StatusModule::status_kind(weapon.module_accessor) == *WEAPON_SNAKE_C4_STATUS_KIND_ESTABLISH_TARGET{
            if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32)
            || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
                StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_SNAKE_C4_STATUS_KIND_DROP_FALL, false);
            }
        }else if StatusModule::status_kind(weapon.module_accessor) == *WEAPON_SNAKE_C4_STATUS_KIND_DROP_FALL {
            StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_SNAKE_C4_STATUS_KIND_ESTABLISH_TARGET, false);
        }
    }
}


#[acmd_script( agent = "snake_c4", script = "game_establishtarget", category = ACMD_GAME )]
unsafe fn snake_c4_target_game(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if SNAKE_FLAG_SPECIAL_LW_SELF_STICK[entry_id] {
            SEARCH(fighter, 0, 0, Hash40::new("rot"), 0.1, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        }else {
            SEARCH(fighter, 0, 0, Hash40::new("rot"), 5.0, 0.0, -3.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        }
    }
}



// #[acmd_script( agent = "snake_c4", script = "game_explosion", category = ACMD_GAME )]
// unsafe fn snake_c4_target_game(fighter : &mut L2CAgentBase) {
//     let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
//     if SNAKE_C4_FLAG_IS_SHOWTIME[entry_id] {
//         SNAKE_C4_FLAG_IS_SHOWTIME[entry_id] = false;
//         if is_excute(fighter) {
//             ATTACK(fighter, 0, 0, Hash40::new("rot"), 17.0, 80, 90, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
//             VisibilityModule::set_whole(fighter.module_accessor, false);
//             QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
//             ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false);
//         }
//         frame(fighter.lua_state_agent, 1.0);
//         if is_excute(fighter) {
//             AttackModule::set_size(fighter.module_accessor, 0, 17);
//             ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false);
//         }
//         if WorkModule::is_flag(fighter.module_accessor, WEAPON_SNAKE_C4*_INSTANCE_WORK_ID_FLAG_GROUND) {
//             frame(fighter.lua_state_agent, 2.0);
//             if is_excute(fighter) {
//                 AttackModule::clear_all(fighter.module_accessor);
//             }
//         }else {
//             frame(fighter.lua_state_agent, 3.0);
//             if is_excute(fighter) {
//                 AttackModule::clear_all(fighter.module_accessor);
//             }
//         }
//     }else {
//         original!(fighter)
//     }
// }





#[status_script(agent = "snake_c4", status = WEAPON_SNAKE_C4_STATUS_KIND_ESTABLISH_TARGET, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn snake_c4_status_pre(weapon: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_SNAKE_C4_DROP_FALL_FLAG, // <-- wrong flag make it check ground collision?
        *WEAPON_STATUS_WORK_KEEP_FLAG_SNAKE_C4_ESTABLISH_TARGET_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_SNAKE_C4_ESTABLISH_TARGET_FLOAT,
        0,
    );
    0.into()
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_PRODUCE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_special_start_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_squat_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    }else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    }
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    }else {
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_produce_main_loop as *const () as _))
}
pub unsafe fn special_lw_produce_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT)
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH)
        || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_DROP.into(), false.into());
            }else {
                fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_ESTABLISH_GROUND.into(), false.into());
            }
            SNAKE_FLAG_SPECIAL_LW_SELF_STICK[entry_id] = true;
        }else {
            SNAKE_FLAG_SPECIAL_LW_SELF_STICK[entry_id] = false;
            fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_ESTABLISH_JUDGE.into(), false.into());
        }
        false.into()
    }else {
        fun_7100018800(fighter, false);
        true.into()
    }
}
pub unsafe fn fun_7100018800(fighter: &mut L2CFighterCommon, skip_check: bool) {
    if skip_check == false {
        if StatusModule::situation_kind(fighter.module_accessor) == StatusModule::prev_situation_kind(fighter.module_accessor) {
            return
        }
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f{x:0.0, y:0.0});
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_PRODUCE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_down_special_start_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_ESTABLISH_GROUND
    && fighter.global_table[0xb].get_i32() != *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_DROP
    && fighter.global_table[0xb].get_i32() != *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_ESTABLISH_JUDGE {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(0));
    }
    0.into()
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_ESTABLISH_GROUND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_special_ground_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if SNAKE_FLAG_SPECIAL_LW_SELF_STICK[entry_id] {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_squat_self_stick"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_self_stick"), 0.0, 1.0, false, 0.0, false, false);
        }
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_START, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    ret
}
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_DROP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_special_air_ground_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if SNAKE_FLAG_SPECIAL_LW_SELF_STICK[entry_id] {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_self_stick"), 0.0, 1.0, false, 0.0, false, false);
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_START, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    ret
}
//new self-stick scripts
#[acmd_script( agent = "snake", script = "game_speciallwselfstick", category = ACMD_GAME )]
unsafe fn snake_down_special_floor(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_STICK_TARGET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
#[acmd_script( agent = "snake", script = "game_speciallwsquatselfstick", category = ACMD_GAME )]
unsafe fn snake_down_special_crouch_floor(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_STICK_TARGET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
#[acmd_script( agent = "snake", script = "game_specialairlwselfstick", category = ACMD_GAME )]
unsafe fn snake_down_special_air_floor(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_STICK_TARGET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
//deletes c4 if not "shot"
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_ESTABLISH_GROUND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_down_special_ground_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    let object = sv_system::battle_object(fighter.lua_state_agent);
    let fighta : *mut Fighter = std::mem::transmute(object);
    if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
    && ArticleModule::motion_kind(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) != hash40("stick_target") {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_DROP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_down_special_air_ground_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    let object = sv_system::battle_object(fighter.lua_state_agent);
    let fighta : *mut Fighter = std::mem::transmute(object);
    if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
    && ArticleModule::motion_kind(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) != hash40("stick_target") {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

// landing back-air ends in squat position
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_landing_attack_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SNAKE_INSTANCE_WORK_FLAG_CYPHER_FALL);
    let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if motion == hash40("attack_air_b") {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_air_b"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT);
    }else if motion == hash40("attack_air_f") {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_air_f"), 0.0, 1.0, false, 0.0, false, false);
    }else if motion == hash40("attack_air_lw") {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    }else if motion == hash40("attack_air_n") {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_air_n"), 0.0, 1.0, false, 0.0, false, false);
    }else if motion == hash40("attack_air_hi") {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_air_hi"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_heavy"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_landing_attack_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_landing_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT) { // added check for back air
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT) { // added check for back air
                fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
            }else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            // true.into()
        }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                // true.into()
            }
        }else {
            return false.into()
        }
    }else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        // true.into()
    }
    true.into()
}
// added self-stick-squat and back-air to down-special check
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_special_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[10].get_i32() == *FIGHTER_STATUS_KIND_SQUAT_WAIT
    || fighter.global_table[10].get_i32() == *FIGHTER_STATUS_KIND_SQUAT_F
    || fighter.global_table[10].get_i32() == *FIGHTER_STATUS_KIND_SQUAT_B
    || fighter.global_table[10].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_DASH
    || fighter.global_table[10].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_LW3
    || MotionModule::motion_kind(fighter.module_accessor) ==  hash40("special_lw_squat_ground")
    || MotionModule::motion_kind(fighter.module_accessor) ==  hash40("special_lw_squat_blast")
    || MotionModule::motion_kind(fighter.module_accessor) ==  hash40("special_lw_squat_self_stick") // added exception for sticking yourself
    || MotionModule::motion_kind(fighter.module_accessor) ==  hash40("landing_air_b") { // added exception for landing back air
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT);
    }else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_down_special_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_down_special_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4) {
        fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_EXPLODING.into(), false.into());
    }else  {
        fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_PRODUCE.into(), false.into());
    }
    1.into()
}

////changed rpg7 side-smash to multi-hit knife
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_side_smash_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    original!(fighter)
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_side_smash_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
    SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = false;
    SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] = 0;
    original!(fighter)
}
//first hit
#[acmd_script( agent = "snake", script = "game_attacks4", category = ACMD_GAME )]
unsafe fn snake_side_smash_game(fighter : &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, false, 0);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 6.0, 361, 25, 0, 20, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.0, 170, 20, 0, 25, 3.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 361, 15, 0, 30, 2.0, 0.0, 5.0, 5.0, Some(0.0), Some(9.0), Some(9.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = true;
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
    }
    frame(fighter.lua_state_agent, 54.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, ArticleOperationTarget(0));
    }
}
#[acmd_script( agent = "snake", script = "expression_attacks4", category = ACMD_EXPRESSION )]
unsafe fn snake_side_smash_expr(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}
#[acmd_script( agent = "snake", script = "sound_attacks4", category = ACMD_SOUND )]
unsafe fn snake_side_smash_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_squat_gear"));
    }
}
#[acmd_script( agent = "snake", script = "effect_attacks4", category = ACMD_EFFECT )]
unsafe fn snake_side_smash_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 4, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 5.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3, 0.11, 0, 0, 0, 0.34, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3.5, 0.05, 0, 180, 0, 0.15, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light2"), Hash40::new("haver"), 0, -0.1, 0, 0, 0, 0, 0.5, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light1"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light2"), false, true);
    }
}
//charge
#[acmd_script( agent = "snake", script = "effect_attacks4charge", category = ACMD_EFFECT )]
unsafe fn snake_side_smash_charge_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    for _ in 0..34 {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, false);
        }
    }
}
//second hit
#[acmd_script( agent = "snake", script = "game_attacks4s2", category = ACMD_GAME )]
unsafe fn snake_side_smash_2_game(fighter : &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        //used extra stun to better link into third hit
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.0, 65, 95, 0, 85, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze_ghost"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.0, 65, 95, 0, 85, 3.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze_ghost"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_paralyze_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 25, false);
        AttackModule::set_paralyze_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 25, false);
        //fake/empty hitboxes to create particle effects
        ATTACK(fighter, 2, 1, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 1, Hash40::new("haver"), 0.0, 0, 0, 0, 0, 3.0, 0.0, 4.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = true;
        // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
        // WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, ArticleOperationTarget(0));
    }
}
#[acmd_script( agent = "snake", script = "expression_attacks4s2", category = ACMD_EXPRESSION )]
unsafe fn snake_side_smash_2_expr(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}
#[acmd_script( agent = "snake", script = "sound_attacks4s2", category = ACMD_SOUND )]
unsafe fn snake_side_smash_2_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_squat_gear"));
    }
}
#[acmd_script( agent = "snake", script = "effect_attacks4s2", category = ACMD_EFFECT )]
unsafe fn snake_side_smash_2_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 4, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 5.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3, 0.11, 0, 0, 0, 0.34, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3.5, 0.05, 0, 180, 0, 0.15, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light2"), Hash40::new("haver"), 0, -0.1, 0, 0, 0, 0, 0.5, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light1"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light2"), false, true);
    }
}
//third hit
#[acmd_script( agent = "snake", script = "game_attacks4s3", category = ACMD_GAME )]
unsafe fn snake_side_smash_3_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("haver"), 7.5, 361, 75, 0, 85, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("haver"), 7.5, 361, 75, 0, 85, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_RPG7, ArticleOperationTarget(0));
    }
}
#[acmd_script( agent = "snake", script = "expression_attacks4s3", category = ACMD_EXPRESSION )]
unsafe fn snake_side_smash_3_expr(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}
#[acmd_script( agent = "snake", script = "sound_attacks4s3", category = ACMD_SOUND )]
unsafe fn snake_side_smash_3_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_squat_gear"));
    }
}
#[acmd_script( agent = "snake", script = "effect_attacks4s3", category = ACMD_EFFECT )]
unsafe fn snake_side_smash_3_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 4, Hash40::new("haver"), 0.0, 0.0, 0.0, Hash40::new("haver"), 0.0, 5.5, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3, 0.11, 0, 0, 0, 0.34, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light1"), Hash40::new("haver"), 0, 3.5, 0.05, 0, 180, 0, 0.15, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_killsword_light2"), Hash40::new("haver"), 0, -0.1, 0, 0, 0, 0, 0.5, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light1"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_killsword_light2"), false, true);
    }
}
////

////changed down-smash to spinning double kick
//attack
#[acmd_script( agent = "snake", script = "game_attacklw4", category = ACMD_GAME )]
unsafe fn snake_down_smash_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 12.0, 25, 87, 0, 30, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("legr"), 12.0, 25, 87, 0, 30, 3.3, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 12.0, 25, 87, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 14.0, 20, 88, 0, 30, 4.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("legl"), 14.0, 20, 88, 0, 30, 3.3, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 14.0, 20, 88, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "snake", script = "sound_attacklw4", category = ACMD_SOUND )]
unsafe fn snake_down_smash_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack_smash_l"));
        PLAY_SE(fighter, Hash40::new("se_snake_smash_l01"));
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack"));
        // PLAY_SE(fighter, Hash40::new("vc_snake_attack04"));
        // PLAY_SEQUENCE(fighter, Hash40::new("seq_snake_rnd_attack_smash_s"));
        PLAY_SE(fighter, Hash40::new("se_snake_smash_l02"));
    }
}
#[acmd_script( agent = "snake", script = "effect_attacklw4", category = ACMD_EFFECT )]
unsafe fn snake_down_smash_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        // EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 13, -1, 35, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 5, 7, 9, 10, -70, 6, 1, true);
    }
    // frame(fighter.lua_state_agent, 13.0);
    // if is_excute(fighter) {
    //     EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, 17, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
    // }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 9, -2, 14, -180, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5, -17, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, true);
    }
}
//charge
#[acmd_script( agent = "snake", script = "expession_attacklw4charge", category = ACMD_EXPRESSION )]
unsafe fn snake_down_smash_charge_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.2, 0.2, -1, 0.7, 0.5, -1, Hash40::new("invalid"));
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, 0);
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, 0);
    }
}
#[acmd_script( agent = "snake", script = "effect_attacklw4charge", category = ACMD_EFFECT )]
unsafe fn snake_down_smash_charge_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    for _ in 0..34 {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 4, 0, 10, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, false);
        }
    }
}
////

////side-special tranq gun
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_side_special_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_s_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_side_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn special_side_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }else {
        fun_7100018800(fighter, false);
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return true.into()
            }
        }
    }
    false.into()
}
//snake
#[acmd_script( agent = "snake", script = "game_specialsstart", category = ACMD_GAME )]
unsafe fn snake_side_special_game(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, false, 0);
    }
    // frame(lua_state_agent, 25.0);
    // if is_excute(fighter) {
    //     FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 8, 4);
    // }
    // frame(lua_state_agent, 27.0);
    // if is_excute(fighter) {
    //     ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, true, *WEAPON_SNAKE_NIKITA_INSTANCE_WORK_ID_FLAG_LIGHT_ON);
    // }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, true, *WEAPON_SNAKE_NIKITA_INSTANCE_WORK_ID_FLAG_SHOOT);
    }
    frame(fighter.lua_state_agent, 83.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, ArticleOperationTarget(0));
    }
}
#[acmd_script( agent = "snake", script = "game_specialairsstart", category = ACMD_GAME )]
unsafe fn snake_side_special_air_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, false, 0);
    }
    // frame(fighter.lua_state_agent, 10.0);
    // if is_excute(fighter) {
    //     GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f{x:0.0, y:3.0});
    // }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, true, *WEAPON_SNAKE_NIKITA_INSTANCE_WORK_ID_FLAG_SHOOT);
    }
    // frame(fighter.lua_state_agent, 70.0);
    // if is_excute(fighter) {
    //     GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f{x:0.0, y:0.0});
    // }
    frame(fighter.lua_state_agent, 83.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_NIKITA, ArticleOperationTarget(0));
    }
}
#[acmd_script( agent = "snake", script = "expression_specialsstart", category = ACMD_EXPRESSION )]
unsafe fn snake_side_special_expr(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
}
#[acmd_script( agent = "snake", script = "expression_specialairsstart", category = ACMD_EXPRESSION )]
unsafe fn snake_side_special_air_expr(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    // frame(lua_state_agent, 25.0);
    // if is_excute(fighter) {
    //     ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false);
    // }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
}
#[acmd_script( agent = "snake", script = "sound_specialsstart", category = ACMD_SOUND )]
unsafe fn snake_side_special_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s07"))
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s01"));
    }
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s02"));
    }
    wait(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s03"));
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s07"))
    }
}#[acmd_script( agent = "snake", script = "sound_specialairsstart", category = ACMD_SOUND )]
unsafe fn snake_side_special_air_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s07"))
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s01"));
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s02"));
    }
    wait(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s03"));
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_s07"))
    }
}
#[acmd_script( agent = "snake", script = "effect_specialsstart", category = ACMD_EFFECT )]
unsafe fn snake_side_special_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -3, 10, 0, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        // EFFECT(fighter, Hash40::new("sys_bananagun_shot"), Hash40::new("haver"), 3, 1, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_bananagun_shot"), Hash40::new("haver"), 0, 0.5, 3, 0, 0, 0, 0.4, true);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        // EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 4.5, 1, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 0, 1, 4.5, 0, 0, 0, 0.2, true);
    }
}
#[acmd_script( agent = "snake", script = "effect_specialairsstart", category = ACMD_EFFECT )]
unsafe fn snake_side_special_air_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -3, 11, -2, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        // EFFECT(fighter, Hash40::new("sys_bananagun_shot"), Hash40::new("haver"), 3, 1, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_bananagun_shot"), Hash40::new("haver"), 0, 0.5, 3, 0, 0, 0, 0.4, true);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        // EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 4.5, 1, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("haver"), 0, 1, 4.5, 0, 0, 0, 0.2, true);
    }
}
//tranq gun
#[acmd_script( agent = "snake_nikita", script = "sound_start", category = ACMD_SOUND )]
unsafe fn snake_tranq_gun_start_snd(_fighter : &mut L2CAgentBase) {
}
#[acmd_script( agent = "snake_nikita", script = "sound_shoot", category = ACMD_SOUND )]
unsafe fn snake_tranq_gun_shoot_snd(_fighter : &mut L2CAgentBase) {
}
//tranq dart
#[acmd_script( agent = "snake_nikitamissile", scripts =  ["game_fly", "game_flyattackcommon", "game_stopfall"], category = ACMD_GAME )]
unsafe fn snake_tranq_dart_fly_game(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 1, 1, Hash40::new("top"), 1.0, 361, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep_ex"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        // ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
    }
}
#[acmd_script( agent = "snake_nikitamissile", scripts = ["sound_fly", "sound_stopfall"], category = ACMD_SOUND )]
unsafe fn snake_tranq_dart_fly_snd(_fighter : &mut L2CAgentBase) {
}
#[acmd_script( agent = "snake_nikitamissile", script = "effect_fly", category = ACMD_EFFECT )]
unsafe fn snake_tranq_dart_fly_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.3, true);
        // LAST_PARTICLE_SET_COLOR(fighter, 0.6, 0.6, 2.8);
        LAST_PARTICLE_SET_COLOR(fighter, 2.5, 2.5, 0.0);
        LAST_EFFECT_SET_RATE(fighter, 0.001);

        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 3, Hash40::new("top"), 0.0, 0.35, -1.5, Hash40::new("haver"), 0.0, -0.25, 1.45, true, Hash40::new("null"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    for _ in 0..5 {
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("snake_missile_smoke"), true, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("snake_missile_smoke2"), true, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("snake_missile_smoke3"), true, true);
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}#[acmd_script( agent = "snake_nikitamissile", script = "effect_stopfall", category = ACMD_EFFECT )]
unsafe fn snake_tranq_dart_fall_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 3, Hash40::new("top"), 0.0, 0.35, -1.5, Hash40::new("haver"), 0.0, -0.25, 1.45, true, Hash40::new("null"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
}
#[acmd_script( agent = "snake_nikitamissile", scripts =  ["game_explosion", "game_fallexplosion", "game_hiexplosion"], category = ACMD_GAME )]
unsafe fn snake_tranq_dart_explode_game(_fighter : &mut L2CAgentBase) {
}
#[acmd_script( agent = "snake_nikitamissile", scripts = ["sound_explosion", "sound_fallexplosion", "sound_hiexplosion"], category = ACMD_SOUND )]
unsafe fn snake_tranq_dart_explode_snd(_fighter : &mut L2CAgentBase) {
}
#[acmd_script( agent = "snake_nikitamissile", scripts = ["effect_explosion", "effect_hiexplosion"], category = ACMD_EFFECT )]
unsafe fn snake_tranq_dart_explode_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    }
}#[acmd_script( agent = "snake_nikitamissile", script = "effect_fallexplosion", category = ACMD_EFFECT )]
unsafe fn snake_tranq_dart_land_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
    }
}
////





////fixed unwanted buffered throws and walking
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_grab_pull_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_pull"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_pull_main_loop as *const () as _))
    // 0.into()
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_DASH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_grab_dash_pull_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_pull"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_pull_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_grab_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return true.into()
    }else if MotionModule::is_end(fighter.module_accessor) {
        if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.1
        || PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.1{
            SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = true;
        }
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
        return true.into()
    }else if MotionModule::frame(fighter.module_accessor) >= 1.0 {
        if fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW != 0 {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }else if fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI != 0 {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        //check stick directly for easier instant f-throw
        }else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.7 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }
        }else if fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B != 0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }
        }else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_ATTACK.into(), false.into());
            return true.into()
        }
    }
    return false.into()
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_grab_attack_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_attack"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_attack_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_grab_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return true.into()
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
        return true.into()
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if ControlModule::get_stick_y(fighter.module_accessor) < -0.7 {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }else if ControlModule::get_stick_y(fighter.module_accessor) > 0.7 {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()

        }else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.7 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }else{
                SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = true;
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
                return true.into()
            }
        }else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.7 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }else{
                SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = true;
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
                return true.into()
            }
        }else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_attack"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    return false.into()
}

// ////fixed buffered throws
// #[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn snake_grab_throw_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
//     if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS) == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI {
//         MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_hi"), 0.0, 1.0, false, 0.0, false, false);
//     }else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS) == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW {
//         MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_lw"), 0.0, 1.0, false, 0.0, false, false);
//     }else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS) == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F {
//         if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
//             MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_f"), 0.0, 1.0, false, 0.0, false, false);
//         }else {
//             SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = true;
//         }
//     }else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS) == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B {
//         if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
//             MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_b"), 0.0, 1.0, false, 0.0, false, false);
//         }else {
//             SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = true;
//         }
//     }
//     fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_throw_main_loop as *const () as _))
//     // 0.into()
// }
// pub unsafe fn snake_grab_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
//     if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
//         fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
//         return true.into()
//     }else if SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] {
//         DamageModule::add_damage(fighter.module_accessor, 0.1, 0);
//         fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
//         return true.into()
//     }else if MotionModule::is_end(fighter.module_accessor) {
//         fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
//         return true.into()
//     }
//     return false.into()
// }

////added grab walk
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_grab_wait_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    ControlModule::reset_trigger(fighter.module_accessor);
    if SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] {
        if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.1 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_f"), 0.0, 1.0, false, 0.0, false, false);
        }else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.1 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_b"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = false;
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
        }
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_wait_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_grab_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return true.into()
    }else if fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW != 0 {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return true.into()
    }else if fighter.global_table[0x21].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI != 0 {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return true.into()
    }else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.7 {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.7 {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_ATTACK.into(), false.into());
            return true.into()
        }
    }
    else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.1 {
        if SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] == false {
            SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = true;
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_b"), 0.0, 1.0, false, 0.0, false, false);
        }
        let walk_speed:f32 = 1.6*(PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor)*-1.0);
        MotionModule::set_rate(fighter.module_accessor, walk_speed);
    }
    else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.1 {
        if SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] == false {
            SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = true;
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_f"), 0.0, 1.0, false, 0.0, false, false);
        }
        let walk_speed:f32 = 1.4*(PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor));
        MotionModule::set_rate(fighter.module_accessor, walk_speed);
    }
    else if SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] {
        SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = false;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    return false.into()
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_grab_wait_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    SNAKE_FLAG_CATCH_WAIT_IS_WALK[entry_id] = false;
    original!(fighter)
}
////






////added new up-taunt and side-taunt
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_taunt_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    if ControlModule::get_command_flag_cat(fighter.module_accessor, 1) == *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_r"), 0.0, 1.0, false, 0.0, false, false);
    }else if ControlModule::get_command_flag_cat(fighter.module_accessor, 1) == *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L
    || ControlModule::get_command_flag_cat(fighter.module_accessor, 1) == *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_taunt_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_taunt_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }else if  MotionModule::is_end(fighter.module_accessor) {
        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("appeal_lw_r") {
            fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT.into(), false.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }else if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX)
    && MotionModule::motion_kind(fighter.module_accessor) != smash::hash40("appeal_lw_r") {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    return false.into()
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_taunt_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT {
        fighter.clear_lua_stack();
        let object = sv_system::battle_object(fighter.lua_state_agent);
        let fighta : *mut Fighter = std::mem::transmute(object);
        if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    return 0.into()
}



// #[acmd_script( agent = "snake", script = "game_appealsr", category = ACMD_GAME )]
// unsafe fn snake_side_taunt_game(fighter : &mut L2CAgentBase) {
//     frame(fighter.lua_state_agent, 29.0);
//     if is_excute(fighter) {
//         ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
//     }
// }
// #[acmd_script( agent = "snake", script = "expression_appealsr", category = ACMD_EXPRESSION )]
// unsafe fn snake_side_taunt_exp(fighter : &mut L2CAgentBase) {
// }
#[acmd_script( agent = "snake", script = "sound_appealsr", category = ACMD_SOUND )]
unsafe fn snake_side_taunt_snd(fighter : &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("vc_snake_win03"));
        }
}
// #[acmd_script( agent = "snake", script = "effect_appealsr", category = ACMD_EFFECT )]
// unsafe fn snake_side_taunt_eff(fighter : &mut L2CAgentBase) {
// }

#[acmd_script( agent = "snake", script = "sound_appealhir", category = ACMD_SOUND )]
unsafe fn snake_up_taunt_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AREA_WIND_2ND_arg10(fighter, 0, 2, 360/*angle*/, 10/*size*/, 1, 0, 12, 30, 30, 80);
        // physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.2, 0.2, -1, 0.7, 0.5, -1, Hash40::new("invalid"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_snake_appealhi"));
    }
}


#[acmd_script( agent = "snake", script = "game_appealendexplode", category = ACMD_GAME )]
unsafe fn snake_down_taunt_explode_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0 );
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, false, 0);
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
        // WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_LW_EXPLODING_FLAG_C4_STARTUP);
        // SNAKE_C4_FLAG_IS_SHOWTIME[entry_id] = true;
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_EXPLOSION, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 90.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
#[acmd_script( agent = "snake", script = "expression_appealendexplode", category = ACMD_EXPRESSION )]
unsafe fn snake_down_taunt_explode_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0 );
    }
    frame(fighter.lua_state_agent, 30.0);
    slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 5, false, 0);
    }
}
#[acmd_script( agent = "snake", script = "sound_appealendexplode", category = ACMD_SOUND )]
unsafe fn snake_down_taunt_explode_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_snake_appealend"));
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_l04"));
        PLAY_SE(fighter, Hash40::new("se_snake_squat"));
    }
    // frame(fighter.lua_state_agent, 70.0);
    // if is_excute(fighter) {
    // }
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_snake_special_l05"));
    }
}
#[acmd_script( agent = "snake", script = "effect_appealendexplode", category = ACMD_EFFECT )]
unsafe fn snake_down_taunt_explode_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}


// #[acmd_script( agent = "snake", script = "expression_appealsr", category = ACMD_EXPRESSION )]
// unsafe fn snake_side_taunt_exp(fighter : &mut L2CAgentBase) {
// }

//added down-taunt box walk and c4 place/explode
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_taunt_wait_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_wait"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
    let appeal_wait_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("appeal_wait_frame"));
    WorkModule::set_int(fighter.module_accessor, appeal_wait_frame, *FIGHTER_SNAKE_STATUS_APPEAL_WORK_INT_WAIT_COUNTER);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    SNAKE_FLAG_APPEAL_LW_C4_EXLPODE[entry_id] = false;
    SNAKE_FLAG_APPEAL_LW_GRENADE_WAIT_COUNT[entry_id] = 0;
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_down_taunt_wait_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_down_taunt_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if SNAKE_FLAG_APPEAL_LW_GRENADE_WAIT_COUNT[entry_id] > 0 {
        SNAKE_FLAG_APPEAL_LW_GRENADE_WAIT_COUNT[entry_id] -= 1;
    }
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_WORK_INT_WAIT_COUNTER);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    || WorkModule::get_int(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_WORK_INT_WAIT_COUNTER) <= 0 {
        fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_APPEAL_END.into(), false.into());
        return true.into()
    //place c4
    }else if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) == *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4) {
            SNAKE_FLAG_APPEAL_LW_C4_EXLPODE[entry_id] = true;
            fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_APPEAL_END.into(), false.into());
        }else {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, false, 0);
            ArticleModule::have(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, Hash40::new("havel"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), 0, false);
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    //spawn grenade
    }else if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) == *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE)
    && SNAKE_FLAG_APPEAL_LW_GRENADE_WAIT_COUNT[entry_id] <= 0
    {
        SNAKE_FLAG_APPEAL_LW_GRENADE_WAIT_COUNT[entry_id] = SNAKE_APPEAL_LW_GRENADE_WAIT_MAX;

        ////adjusts first grenade position only
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, 0);
        PLAY_SE(fighter, Hash40::new("se_snake_special_n01"));
        ArticleModule::have(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, Hash40::new("havel"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), 0, false);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        // let pos_x = PostureModule::pos_x(fighter.module_accessor);
        // let pos_y = PostureModule::pos_y(fighter.module_accessor);
        // ArticleModule::set_pos(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, Vector3f{x:pos_x, y:pos_y+1.0, z:0.0});

        ////doesn't set player as owner of grenade
        // ItemModule::attach_item(fighter.module_accessor, ItemKind(*ITEM_KIND_SNAKEGRENADE), *ATTACH_ITEM_GROUP_BODY, true);
        // EFFECT_OFF_KIND(fighter, Hash40::new("sys_item_get"), true, true);
        // PLAY_SE(fighter, Hash40::new("se_snake_special_n01"));
        // ItemModule::drop_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_SNAKEGRENADE), 0.0, 0.0);

        // DamageModule::add_damage(fighter.module_accessor, *LINK_NO_ARTICLE as f32, 0);

        ////adjusts first article only
        // ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, 0);
        // let article_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CONSTRAINT);
        // let article_boma = sv_battle_object::module_accessor(article_id as u32);
        // LinkModule::set_model_constraint_target_joint(article_boma, Hash40::new("kneer"));
    }else {
        let velocity_x :f32 = PostureModule::lr(fighter.module_accessor) * ControlModule::get_stick_x(fighter.module_accessor);
        SET_SPEED_EX(fighter, velocity_x*0.6, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    return false.into()
}
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_down_taunt_wait_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SNAKE_STATUS_KIND_APPEAL_END {
        fighter.clear_lua_stack();
        let object = sv_system::battle_object(fighter.lua_state_agent);
        let fighta : *mut Fighter = std::mem::transmute(object);
        if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    return 0.into()
}
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_APPEAL_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_taunt_end_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if SNAKE_FLAG_APPEAL_LW_C4_EXLPODE[entry_id] {
        SNAKE_FLAG_APPEAL_LW_C4_EXLPODE[entry_id] = false;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_end_explode"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_down_taunt_end_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_down_taunt_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    return false.into()
}
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_APPEAL_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_down_taunt_end_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    let object = sv_system::battle_object(fighter.lua_state_agent);
    let fighta : *mut Fighter = std::mem::transmute(object);
    if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return 0.into()
}
////




pub fn install() {
    smashline::install_status_scripts!(
        snake_c4_status_pre,
        // snake_c4_fall_status_main,
        snake_down_special_start_status_main,
        snake_down_special_start_status_end,
        snake_down_special_ground_status_main,
        snake_down_special_air_ground_status_main,
        snake_down_special_ground_status_end,
        snake_down_special_air_ground_status_end,
        snake_down_special_status_main,
        snake_landing_attack_status_main,
        snake_side_smash_status_main,
        snake_side_smash_status_end,
        snake_side_special_status_main,
        // snake_side_special_status_end
        snake_grab_pull_status_main,
        snake_grab_dash_pull_status_main,
        snake_grab_attack_status_main,
        // snake_grab_throw_status_main,
        snake_grab_wait_status_main,
        snake_grab_wait_status_end,
        snake_taunt_status_main,
        snake_taunt_status_end,
        snake_down_taunt_wait_status_main,
        snake_down_taunt_wait_status_end,
        snake_down_taunt_end_status_main,
        snake_down_taunt_end_status_end
    );
    smashline::install_agent_frames!(
        snake_frame,
        snake_c4_frame
    );
    smashline::install_acmd_scripts!(
        snake_c4_target_game,
        // snake_c4_fall_game,
        // snake_c4_fall_sound,
        // snake_c4_fall_effect,
        snake_down_special_floor,
        snake_down_special_crouch_floor,
        snake_down_special_air_floor,

        snake_side_smash_game,
        snake_side_smash_expr,
        snake_side_smash_snd,
        snake_side_smash_eff,

        snake_side_smash_charge_eff,

        snake_side_smash_2_game,
        snake_side_smash_2_expr,
        snake_side_smash_2_snd,
        snake_side_smash_2_eff,

        snake_side_smash_3_game,
        snake_side_smash_3_expr,
        snake_side_smash_3_snd,
        snake_side_smash_3_eff,

        snake_side_special_game,
        snake_side_special_air_game,
        snake_side_special_expr,
        snake_side_special_air_expr,
        snake_side_special_snd,
        snake_side_special_air_snd,
        snake_side_special_eff,
        snake_side_special_air_eff,

        snake_down_smash_game,
        snake_down_smash_snd,
        snake_down_smash_eff,

        snake_down_smash_charge_exp,
        snake_down_smash_charge_eff,

        snake_tranq_gun_start_snd,
        snake_tranq_gun_shoot_snd,

        snake_tranq_dart_fly_game,
        snake_tranq_dart_fly_snd,
        snake_tranq_dart_fly_eff,
        snake_tranq_dart_fall_eff,
        snake_tranq_dart_explode_game,
        snake_tranq_dart_explode_snd,
        snake_tranq_dart_explode_eff,
        snake_tranq_dart_land_eff,

        // snake_side_taunt_game,
        // snake_side_taunt_exp,
        snake_side_taunt_snd,
        // snake_side_taunt_eff

        snake_up_taunt_snd,

        snake_down_taunt_explode_game,
        snake_down_taunt_explode_exp,
        snake_down_taunt_explode_snd,
        snake_down_taunt_explode_eff
    );
}
