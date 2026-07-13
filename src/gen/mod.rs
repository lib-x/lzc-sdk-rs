// @generated
pub mod cloud {
    pub mod lazycat {
        pub mod apis {
            // @@protoc_insertion_point(attribute:cloud.lazycat.apis.common)
            pub mod common {
                include!("cloud/lazycat/apis/common/cloud.lazycat.apis.common.rs");
                // @@protoc_insertion_point(cloud.lazycat.apis.common)
            }
            // @@protoc_insertion_point(attribute:cloud.lazycat.apis.localdevice)
            pub mod localdevice {
                include!("cloud/lazycat/apis/localdevice/cloud.lazycat.apis.localdevice.rs");
                // @@protoc_insertion_point(cloud.lazycat.apis.localdevice)
            }
            // @@protoc_insertion_point(attribute:cloud.lazycat.apis.sys)
            pub mod sys {
                include!("cloud/lazycat/apis/sys/cloud.lazycat.apis.sys.rs");
                // @@protoc_insertion_point(cloud.lazycat.apis.sys)
            }
        }
    }
}
pub mod io {
    pub mod containerd {
        pub mod cgroups {
            // @@protoc_insertion_point(attribute:io.containerd.cgroups.v2)
            pub mod v2 {
                include!("io/containerd/cgroups/v2/io.containerd.cgroups.v2.rs");
                // @@protoc_insertion_point(io.containerd.cgroups.v2)
            }
        }
    }
}
pub mod lzc {
    // @@protoc_insertion_point(attribute:lzc.dlna)
    pub mod dlna {
        include!("lzc/dlna/lzc.dlna.rs");
        // @@protoc_insertion_point(lzc.dlna)
    }
}
