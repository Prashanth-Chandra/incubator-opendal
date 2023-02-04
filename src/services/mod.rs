// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Providing specific services support.
//!
//! In order to implement a service, we need the following things:
//!
//! - Builder: responsible for building the service backend.
//! - Backend: the service backend which implements the [`Accessor`][crate::raw::Accessor] trait.

mod azblob;
pub use azblob::AzblobBuilder as Azblob;

mod azdfs;
pub use azdfs::AzdfsBuilder as Azdfs;

mod fs;
pub use fs::FsBuilder as Fs;

#[cfg(feature = "services-ftp")]
mod ftp;
#[cfg(feature = "services-ftp")]
pub use ftp::FtpBuilder as Ftp;

mod gcs;
pub use gcs::GcsBuilder as Gcs;

mod ghac;
pub use ghac::GhacBuilder as Ghac;

#[cfg(feature = "services-hdfs")]
mod hdfs;
#[cfg(feature = "services-hdfs")]
pub use hdfs::HdfsBuilder as Hdfs;

mod http;
pub use self::http::HttpBuilder as Http;

#[cfg(feature = "services-ipfs")]
mod ipfs;
#[cfg(feature = "services-ipfs")]
pub use self::ipfs::IpfsBuilder as Ipfs;

mod ipmfs;
pub use ipmfs::IpmfsBuilder as Ipmfs;

#[cfg(feature = "services-memcached")]
mod memcached;
#[cfg(feature = "services-memcached")]
pub use memcached::MemcachedBuilder as Memcached;

mod memory;
pub use memory::MemoryBuilder as Memory;

#[cfg(feature = "services-moka")]
mod moka;
#[cfg(feature = "services-moka")]
pub use self::moka::MokaBuilder as Moka;

mod obs;
pub use obs::ObsBuilder as Obs;

mod oss;
pub use oss::OssBuilder as Oss;

#[cfg(feature = "services-redis")]
mod redis;
#[cfg(feature = "services-redis")]
pub use self::redis::RedisBuilder as Redis;

#[cfg(feature = "services-rocksdb")]
mod rocksdb;
#[cfg(feature = "services-rocksdb")]
pub use self::rocksdb::RocksdbBuilder as Rocksdb;

mod s3;
pub use s3::S3Builder as S3;

mod webdav;
pub use webdav::WebdavBuilder as Webdav;
