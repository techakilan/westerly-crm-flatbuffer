// automatically generated by the FlatBuffers compiler, do not modify

use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod users {

  use std::cmp::Ordering;
  use std::mem;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

  pub enum UserOffset {}
  #[derive(Copy, Clone, PartialEq)]

  pub struct User<'a> {
    pub _tab: flatbuffers::Table<'a>,
  }

  impl<'a> flatbuffers::Follow<'a> for User<'a> {
    type Inner = User<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
      Self {
        _tab: flatbuffers::Table { buf, loc },
      }
    }
  }

  impl<'a> User<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
      User { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
      _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
      args: &'args UserArgs<'args>,
    ) -> flatbuffers::WIPOffset<User<'bldr>> {
      let mut builder = UserBuilder::new(_fbb);
      builder.add_id(args.id);
      if let Some(x) = args.name {
        builder.add_name(x);
      }
      builder.finish()
    }

    pub const VT_NAME: flatbuffers::VOffsetT = 4;
    pub const VT_ID: flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn name(&self) -> Option<&'a str> {
      self
        ._tab
        .get::<flatbuffers::ForwardsUOffset<&str>>(User::VT_NAME, None)
    }
    #[inline]
    pub fn id(&self) -> u64 {
      self._tab.get::<u64>(User::VT_ID, Some(0)).unwrap()
    }
  }

  impl flatbuffers::Verifiable for User<'_> {
    #[inline]
    fn run_verifier(
      v: &mut flatbuffers::Verifier,
      pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
      use self::flatbuffers::Verifiable;
      v.visit_table(pos)?
        .visit_field::<flatbuffers::ForwardsUOffset<&str>>(&"name", Self::VT_NAME, false)?
        .visit_field::<u64>(&"id", Self::VT_ID, false)?
        .finish();
      Ok(())
    }
  }
  pub struct UserArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub id: u64,
  }
  impl<'a> Default for UserArgs<'a> {
    #[inline]
    fn default() -> Self {
      UserArgs { name: None, id: 0 }
    }
  }
  pub struct UserBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
  }
  impl<'a: 'b, 'b> UserBuilder<'a, 'b> {
    #[inline]
    pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b str>) {
      self
        .fbb_
        .push_slot_always::<flatbuffers::WIPOffset<_>>(User::VT_NAME, name);
    }
    #[inline]
    pub fn add_id(&mut self, id: u64) {
      self.fbb_.push_slot::<u64>(User::VT_ID, id, 0);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> UserBuilder<'a, 'b> {
      let start = _fbb.start_table();
      UserBuilder {
        fbb_: _fbb,
        start_: start,
      }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<User<'a>> {
      let o = self.fbb_.end_table(self.start_);
      flatbuffers::WIPOffset::new(o.value())
    }
  }

  impl std::fmt::Debug for User<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let mut ds = f.debug_struct("User");
      ds.field("name", &self.name());
      ds.field("id", &self.id());
      ds.finish()
    }
  }
  #[inline]
  #[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
  pub fn get_root_as_user<'a>(buf: &'a [u8]) -> User<'a> {
    unsafe { flatbuffers::root_unchecked::<User<'a>>(buf) }
  }

  #[inline]
  #[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
  pub fn get_size_prefixed_root_as_user<'a>(buf: &'a [u8]) -> User<'a> {
    unsafe { flatbuffers::size_prefixed_root_unchecked::<User<'a>>(buf) }
  }

  #[inline]
  /// Verifies that a buffer of bytes contains a `User`
  /// and returns it.
  /// Note that verification is still experimental and may not
  /// catch every error, or be maximally performant. For the
  /// previous, unchecked, behavior use
  /// `root_as_user_unchecked`.
  pub fn root_as_user(buf: &[u8]) -> Result<User, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<User>(buf)
  }
  #[inline]
  /// Verifies that a buffer of bytes contains a size prefixed
  /// `User` and returns it.
  /// Note that verification is still experimental and may not
  /// catch every error, or be maximally performant. For the
  /// previous, unchecked, behavior use
  /// `size_prefixed_root_as_user_unchecked`.
  pub fn size_prefixed_root_as_user(buf: &[u8]) -> Result<User, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<User>(buf)
  }
  #[inline]
  /// Verifies, with the given options, that a buffer of bytes
  /// contains a `User` and returns it.
  /// Note that verification is still experimental and may not
  /// catch every error, or be maximally performant. For the
  /// previous, unchecked, behavior use
  /// `root_as_user_unchecked`.
  pub fn root_as_user_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
  ) -> Result<User<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<User<'b>>(opts, buf)
  }
  #[inline]
  /// Verifies, with the given verifier options, that a buffer of
  /// bytes contains a size prefixed `User` and returns
  /// it. Note that verification is still experimental and may not
  /// catch every error, or be maximally performant. For the
  /// previous, unchecked, behavior use
  /// `root_as_user_unchecked`.
  pub fn size_prefixed_root_as_user_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
  ) -> Result<User<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<User<'b>>(opts, buf)
  }
  #[inline]
  /// Assumes, without verification, that a buffer of bytes contains a User and returns it.
  /// # Safety
  /// Callers must trust the given bytes do indeed contain a valid `User`.
  pub unsafe fn root_as_user_unchecked(buf: &[u8]) -> User {
    flatbuffers::root_unchecked::<User>(buf)
  }
  #[inline]
  /// Assumes, without verification, that a buffer of bytes contains a size prefixed User and returns it.
  /// # Safety
  /// Callers must trust the given bytes do indeed contain a valid size prefixed `User`.
  pub unsafe fn size_prefixed_root_as_user_unchecked(buf: &[u8]) -> User {
    flatbuffers::size_prefixed_root_unchecked::<User>(buf)
  }
  #[inline]
  pub fn finish_user_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<User<'a>>,
  ) {
    fbb.finish(root, None);
  }

  #[inline]
  pub fn finish_size_prefixed_user_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<User<'a>>,
  ) {
    fbb.finish_size_prefixed(root, None);
  }
} // pub mod users
