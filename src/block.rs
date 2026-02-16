use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block(u8);

impl Block {
  pub const CUBE: Self = Self(BlockKind::Cube.index());
  pub const SLOPE: Self = Self(BlockKind::Slope.index());
  pub const CORNER: Self = Self(BlockKind::Corner.index());
  pub const PYRAMID: Self = Self(BlockKind::Pyramid.index());
  pub const INV_CORNER: Self = Self(BlockKind::InvCorner.index());

  pub const fn from_raw(raw: u8) -> Option<Self> {
    if raw < BLOCK_KINDS.len() as u8 {
      Some(Self(raw))
    } else {
      None
    }
  }

  pub const fn to_raw(&self) -> u8 {
    self.0
  }

  pub const fn kind(&self) -> BlockKind {
    BLOCK_KINDS[self.0 as usize]
  }

  pub const fn up(&self) -> Direction {
    CUBE_UP_FORWARDS[BLOCK_TO_CUBE[self.0 as usize] as usize].0
  }

  pub const fn forward(&self) -> Direction {
    CUBE_UP_FORWARDS[BLOCK_TO_CUBE[self.0 as usize] as usize].1
  }

  pub const fn rotation(&self) -> Quat {
    CUBE_ROTATIONS[BLOCK_TO_CUBE[self.0 as usize] as usize]
  }

  pub fn rotate_by(&self, axis: Direction) -> Self {
    let kind = self.kind();

    if kind == BlockKind::Cube {
      return *self;
    }

    let index = kind.index() as usize;
    let rotations = kind.rotations() as usize;
    let mut up = self.up();
    let mut forward = self.forward();

    loop {
      up = up.rotate_around(axis);
      forward = forward.rotate_around(axis);

      if let Some(offset) = BLOCK_TO_CUBE[index..index + rotations]
        .iter()
        .map(|&i| CUBE_UP_FORWARDS[i as usize])
        .position(|axes| {
          axes == (up, forward)
            || matches!(kind, BlockKind::Slope if axes == (forward, up))
        })
      {
        break Self((index + offset) as u8);
      }
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BlockKind {
  #[default]
  Cube,
  Slope,
  Corner,
  Pyramid,
  InvCorner,
}

impl BlockKind {
  pub const ALL: [Self; 5] = [
    Self::Cube,
    Self::Slope,
    Self::Corner,
    Self::Pyramid,
    Self::InvCorner,
  ];

  pub const fn index(&self) -> u8 {
    match self {
      Self::Cube => 0,
      Self::Slope => Self::Cube.index() + Self::Cube.rotations(),
      Self::Corner => Self::Slope.index() + Self::Slope.rotations(),
      Self::Pyramid => Self::Corner.index() + Self::Corner.rotations(),
      Self::InvCorner => Self::Pyramid.index() + Self::Pyramid.rotations(),
    }
  }

  pub const fn rotations(&self) -> u8 {
    match self {
      Self::Cube => 1,
      Self::Slope => 12,
      Self::Corner => 8,
      Self::Pyramid => 24,
      Self::InvCorner => 8,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
  X,
  Y,
  Z,
  NegX,
  NegY,
  NegZ,
}

impl Direction {
  pub const ALL: [Self; 6] = [
    Self::X,
    Self::Y,
    Self::Z,
    Self::NegX,
    Self::NegY,
    Self::NegZ,
  ];

  pub const fn inverse(&self) -> Self {
    match self {
      Self::X => Self::NegX,
      Self::Y => Self::NegY,
      Self::Z => Self::NegZ,
      Self::NegX => Self::X,
      Self::NegY => Self::Y,
      Self::NegZ => Self::Z,
    }
  }

  pub const fn rotate_around(&self, axis: Direction) -> Self {
    match (self, axis) {
      (Self::X, Self::Y) => Self::Z,
      (Self::X, Self::Z) => Self::NegY,
      (Self::X, Self::NegY) => Self::NegZ,
      (Self::X, Self::NegZ) => Self::Y,
      (Self::NegX, Self::Y) => Self::NegZ,
      (Self::NegX, Self::NegZ) => Self::NegY,
      (Self::NegX, Self::NegY) => Self::Z,
      (Self::NegX, Self::Z) => Self::Y,

      (Self::Y, Self::Z) => Self::X,
      (Self::Y, Self::X) => Self::NegZ,
      (Self::Y, Self::NegZ) => Self::NegX,
      (Self::Y, Self::NegX) => Self::Z,
      (Self::NegY, Self::Z) => Self::NegX,
      (Self::NegY, Self::NegX) => Self::NegZ,
      (Self::NegY, Self::NegZ) => Self::X,
      (Self::NegY, Self::X) => Self::Z,

      (Self::Z, Self::X) => Self::Y,
      (Self::Z, Self::Y) => Self::NegX,
      (Self::Z, Self::NegX) => Self::NegY,
      (Self::Z, Self::NegY) => Self::X,
      (Self::NegZ, Self::X) => Self::NegY,
      (Self::NegZ, Self::NegY) => Self::NegX,
      (Self::NegZ, Self::NegX) => Self::Y,
      (Self::NegZ, Self::Y) => Self::X,

      (Self::X, Self::X) => axis,
      (Self::Y, Self::Y) => axis,
      (Self::Z, Self::Z) => axis,
      (Self::NegX, Self::NegX) => axis,
      (Self::NegY, Self::NegY) => axis,
      (Self::NegZ, Self::NegZ) => axis,
      (Self::X, Self::NegX) => axis,
      (Self::Y, Self::NegY) => axis,
      (Self::Z, Self::NegZ) => axis,
      (Self::NegX, Self::X) => axis,
      (Self::NegY, Self::Y) => axis,
      (Self::NegZ, Self::Z) => axis,
    }
  }
}

static BLOCK_KINDS: [BlockKind; 53] = [
  // Cube.
  BlockKind::Cube,
  // Slope.
  BlockKind::Slope,
  BlockKind::Slope,
  BlockKind::Slope,
  BlockKind::Slope,
  BlockKind::Slope,
  BlockKind::Slope,
  BlockKind::Slope,
  BlockKind::Slope,
  BlockKind::Slope,
  BlockKind::Slope,
  BlockKind::Slope,
  BlockKind::Slope,
  // Corner.
  BlockKind::Corner,
  BlockKind::Corner,
  BlockKind::Corner,
  BlockKind::Corner,
  BlockKind::Corner,
  BlockKind::Corner,
  BlockKind::Corner,
  BlockKind::Corner,
  // Pyramid.
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  BlockKind::Pyramid,
  // Inverse Corner.
  BlockKind::InvCorner,
  BlockKind::InvCorner,
  BlockKind::InvCorner,
  BlockKind::InvCorner,
  BlockKind::InvCorner,
  BlockKind::InvCorner,
  BlockKind::InvCorner,
  BlockKind::InvCorner,
];

#[rustfmt::skip]
static BLOCK_TO_CUBE: [u8; BLOCK_KINDS.len()] = [
  // Cube.
  0,
  // Slope.
  0, 1, 2, 3, 4, 5, 6, 7, 12, 13, 14, 15,
  // Corner.
  0, 1, 2, 3, 4, 5, 6, 7,
  // Pyramid.
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
  21, 22, 23,
  // Inverse Corner.
  0, 1, 2, 3, 8, 9, 10, 11,
];

static CUBE_UP_FORWARDS: [(Direction, Direction); 24] = [
  (Direction::Y, Direction::Z),
  (Direction::NegZ, Direction::Y),
  (Direction::NegY, Direction::NegZ),
  (Direction::Z, Direction::NegY),
  (Direction::X, Direction::Z),
  (Direction::X, Direction::Y),
  (Direction::X, Direction::NegZ),
  (Direction::X, Direction::NegY),
  (Direction::NegY, Direction::Z),
  (Direction::Y, Direction::NegZ),
  (Direction::Y, Direction::NegZ),
  (Direction::Z, Direction::Y),
  (Direction::NegX, Direction::Z),
  (Direction::NegX, Direction::NegY),
  (Direction::NegX, Direction::NegZ),
  (Direction::NegX, Direction::Y),
  (Direction::Y, Direction::X),
  (Direction::Z, Direction::X),
  (Direction::NegY, Direction::X),
  (Direction::NegZ, Direction::X),
  (Direction::NegY, Direction::NegX),
  (Direction::Z, Direction::NegX),
  (Direction::Y, Direction::NegX),
  (Direction::NegZ, Direction::NegX),
];

static CUBE_ROTATIONS: [Quat; 24] = [
  // Mat3::from_cols_array_2d(&[
  //   [1.0, 0.0, 0.0],
  //   [0.0, 1.0, 0.0],
  //   [0.0, 0.0, 1.0],
  // ]),
  Quat::from_array([0.0, 0.0, 0.0, 1.0]),
  // Mat3::from_cols_array_2d(&[
  //   [1.0, 0.0, 0.0],
  //   [0.0, 0.0, -1.0],
  //   [0.0, 1.0, 0.0],
  // ]),
  Quat::from_array([0.70710677, 0.0, 0.0, -0.70710677]),
  // Mat3::from_cols_array_2d(&[
  //   [1.0, 0.0, 0.0],
  //   [0.0, -1.0, 0.0],
  //   [0.0, 0.0, -1.0],
  // ]),
  Quat::from_array([1.0, 0.0, 0.0, 0.0]),
  // Mat3::from_cols_array_2d(&[
  //   [1.0, 0.0, 0.0],
  //   [0.0, 0.0, 1.0],
  //   [0.0, -1.0, 0.0],
  // ]),
  Quat::from_array([0.70710677, 0.0, 0.0, 0.70710677]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, -1.0, 0.0],
  //   [1.0, 0.0, 0.0],
  //   [0.0, 0.0, 1.0],
  // ]),
  Quat::from_array([0.0, 0.0, 0.70710677, -0.70710677]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, 0.0, 1.0],
  //   [1.0, 0.0, 0.0],
  //   [0.0, 1.0, 0.0],
  // ]),
  Quat::from_array([0.5, 0.5, 0.5, -0.5]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, 1.0, 0.0],
  //   [1.0, 0.0, 0.0],
  //   [0.0, 0.0, -1.0],
  // ]),
  Quat::from_array([0.70710677, 0.70710677, 0.0, 0.0]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, 0.0, -1.0],
  //   [1.0, 0.0, 0.0],
  //   [0.0, -1.0, 0.0],
  // ]),
  Quat::from_array([0.5, 0.5, -0.5, 0.5]),
  // Mat3::from_cols_array_2d(&[
  //   [-1.0, 0.0, 0.0],
  //   [0.0, -1.0, 0.0],
  //   [0.0, 0.0, 1.0],
  // ]),
  Quat::from_array([0.0, 0.0, 1.0, 0.0]),
  // Mat3::from_cols_array_2d(&[
  //   [-1.0, 0.0, 0.0],
  //   [0.0, 1.0, 0.0],
  //   [0.0, 0.0, -1.0],
  // ]),
  Quat::from_array([0.0, 1.0, 0.0, 0.0]),
  // Mat3::from_cols_array_2d(&[
  //   [-1.0, 0.0, 0.0],
  //   [0.0, 1.0, 0.0],
  //   [0.0, 0.0, -1.0],
  // ]),
  Quat::from_array([0.0, 1.0, 0.0, 0.0]),
  // Mat3::from_cols_array_2d(&[
  //   [-1.0, 0.0, 0.0],
  //   [0.0, 0.0, 1.0],
  //   [0.0, 1.0, 0.0],
  // ]),
  Quat::from_array([0.0, 0.70710677, 0.70710677, 0.0]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, 1.0, 0.0],
  //   [-1.0, 0.0, 0.0],
  //   [0.0, 0.0, 1.0],
  // ]),
  Quat::from_array([0.0, 0.0, 0.70710677, 0.70710677]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, 0.0, 1.0],
  //   [-1.0, 0.0, 0.0],
  //   [0.0, -1.0, 0.0],
  // ]),
  Quat::from_array([0.5, -0.5, 0.5, 0.5]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, -1.0, 0.0],
  //   [-1.0, 0.0, 0.0],
  //   [0.0, 0.0, -1.0],
  // ]),
  Quat::from_array([0.70710677, -0.70710677, 0.0, 0.0]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, 0.0, -1.0],
  //   [-1.0, 0.0, 0.0],
  //   [0.0, 1.0, 0.0],
  // ]),
  Quat::from_array([0.5, -0.5, -0.5, -0.5]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, 0.0, -1.0],
  //   [0.0, 1.0, 0.0],
  //   [1.0, 0.0, 0.0],
  // ]),
  Quat::from_array([0.0, 0.70710677, 0.0, 0.70710677]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, 1.0, 0.0],
  //   [0.0, 0.0, 1.0],
  //   [1.0, 0.0, 0.0],
  // ]),
  Quat::from_array([0.5, 0.5, 0.5, 0.5]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, 0.0, 1.0],
  //   [0.0, -1.0, 0.0],
  //   [1.0, 0.0, 0.0],
  // ]),
  Quat::from_array([0.70710677, 0.0, 0.70710677, 0.0]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, -1.0, 0.0],
  //   [0.0, 0.0, -1.0],
  //   [1.0, 0.0, 0.0],
  // ]),
  Quat::from_array([0.5, -0.5, 0.5, -0.5]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, 0.0, -1.0],
  //   [0.0, -1.0, 0.0],
  //   [-1.0, 0.0, 0.0],
  // ]),
  Quat::from_array([0.70710677, 0.0, -0.70710677, 0.0]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, -1.0, 0.0],
  //   [0.0, 0.0, 1.0],
  //   [-1.0, 0.0, 0.0],
  // ]),
  Quat::from_array([0.5, -0.5, -0.5, 0.5]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, 0.0, 1.0],
  //   [0.0, 1.0, 0.0],
  //   [-1.0, 0.0, 0.0],
  // ]),
  Quat::from_array([0.0, 0.70710677, 0.0, -0.70710677]),
  // Mat3::from_cols_array_2d(&[
  //   [0.0, 1.0, 0.0],
  //   [0.0, 0.0, -1.0],
  //   [-1.0, 0.0, 0.0],
  // ]),
  Quat::from_array([0.5, 0.5, -0.5, -0.5]),
];
