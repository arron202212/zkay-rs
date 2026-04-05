// package org.bouncycastle.crypto;

// import java.security.Permission;
// import java.util.HashSet;
// import java.util.Set;

// /**
//  * Permissions that need to be configured if a SecurityManager is used.
//  */
// public class CryptoServicesPermission
//     extends Permission
// {
//     /**
//      * Enable the setting of global configuration properties. This permission implies THREAD_LOCAL_CONFIG
//      */
//     public static final String GLOBAL_CONFIG = "globalConfig";

//     /**
//      * Enable the setting of thread local configuration properties.
//      */
//     public static final String THREAD_LOCAL_CONFIG = "threadLocalConfig";

//     /**
//      * Enable the setting of the default SecureRandom.
//      */
//     public static final String DEFAULT_RANDOM = "defaultRandomConfig";

//     private final Set<String> actions = new HashSet<String>();

//     public CryptoServicesPermission(String name)
//     {
//         super(name);

//         this.actions.add(name);
//     }

//     public boolean implies(Permission permission)
//     {
//         if (permission instanceof CryptoServicesPermission)
//         {
//             CryptoServicesPermission other = (CryptoServicesPermission)permission;

//             if (this.getName().equals(other.getName()))
//             {
//                 return true;
//             }

//             if (this.actions.containsAll(other.actions))
//             {
//                 return true;
//             }
//         }

//         return false;
//     }

//     public boolean equals(Object obj)
//     {
//         if (obj instanceof CryptoServicesPermission)
//         {
//             CryptoServicesPermission other = (CryptoServicesPermission)obj;

//             if (this.actions.equals(other.actions))
//             {
//                 return true;
//             }
//         }

//         return false;
//     }

//     public int hashCode()
//     {
//         return actions.hashCode();
//     }

//     public String getActions()
//     {
//         return actions.toString();
//     }
// }
use std::collections::HashSet;

/// 對應 Java 的 CryptoServicesPermission
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptoServicesPermission {
    // 權限名稱，對標 Java 的 getName()
    name: String,
    // 具體的動作集合
    actions: HashSet<String>,
}

// 定義常量
pub const GLOBAL_CONFIG: &str = "globalConfig";
pub const THREAD_LOCAL_CONFIG: &str = "threadLocalConfig";
pub const DEFAULT_RANDOM: &str = "defaultRandomConfig";

impl CryptoServicesPermission {
    pub fn new(name: &str) -> Self {
        let mut actions = HashSet::new();
        actions.insert(name.to_string());
        
        Self {
            name: name.to_string(),
            actions,
        }
    }

    /// 對應 Java 的 implies 邏輯
    /// 判斷當前權限是否「包含」另一個權限
    pub fn implies(&self, other: &CryptoServicesPermission) -> bool {
        // 如果名稱相同，直接返回 true
        if self.name == other.name {
            return true;
        }

        // 檢查 GLOBAL_CONFIG 是否包含 THREAD_LOCAL_CONFIG 的特殊邏輯
        if self.actions.contains(GLOBAL_CONFIG) && other.actions.contains(THREAD_LOCAL_CONFIG) {
            return true;
        }

        // 通用邏輯：檢查是否包含對方的所有 actions
        other.actions.is_subset(&self.actions)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_actions_string(&self) -> String {
        format!("{:?}", self.actions)
    }
}
use bitflags::bitflags;

bitflags! {
    pub struct CryptoPermissions: u32 {
        const THREAD_LOCAL_CONFIG = 0b0001;
        const DEFAULT_RANDOM      = 0b0010;
        // GLOBAL_CONFIG 包含 THREAD_LOCAL_CONFIG
        const GLOBAL_CONFIG       = 0b0100 | Self::THREAD_LOCAL_CONFIG.bits();
    }
}

// 使用範例：
// let my_perm = CryptoPermissions::GLOBAL_CONFIG;
// assert!(my_perm.contains(CryptoPermissions::THREAD_LOCAL_CONFIG));
