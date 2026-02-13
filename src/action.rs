use bevy::prelude::*;

use crate::Selected;

#[derive(Default)]
pub struct ActionPlugin;

impl Plugin for ActionPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_message::<ActionMessage>()
      .init_resource::<ActionHistory>()
      .add_systems(PostUpdate, consume_actions_messages);
  }
}

#[derive(Default, Resource)]
pub struct ActionHistory {
  history: Vec<BoxedAction>,
  current: usize,
}

impl ActionHistory {
  pub fn clear(&mut self) {
    self.history.clear();
    self.current = 0;
  }

  fn push(&mut self, action: BoxedAction, world: &mut World) {
    match action.redo(world) {
      // TODO: Is there a way to make this nicely actionable?
      ActionResult::Failed => {
        warn!("Could not push action. There may be more information above")
      }
      ActionResult::Success => {
        self.history.drain(self.current..);
        self.history.push(action);
        self.current = self.history.len();
      }
    }
  }

  fn redo(&mut self, world: &mut World) {
    if let Some(action) = self.history.get(self.current) {
      match action.redo(world) {
        // TODO: Is there a way to make this nicely actionable?
        ActionResult::Failed => {
          warn!("Could not redo action. There may be more information above")
        }
        ActionResult::Success => self.current += 1,
      }
    }
  }

  fn undo(&mut self, world: &mut World) {
    if let Some(action) = self
      .current
      .checked_sub(1)
      .and_then(|current| self.history.get(current))
    {
      match action.undo(world) {
        // TODO: Is there a way to make this nicely actionable?
        ActionResult::Failed => {
          warn!("Could not undo action. There may be more information above")
        }
        ActionResult::Success => self.current -= 1,
      }
    }
  }
}

/// A [`Message`] used to interact with the [`ActionHistory`].
#[derive(Message)]
pub enum ActionMessage {
  /// Pushes a new action, overwritting any actions that could have been redone.
  Push(BoxedAction),
  /// Redoes an undone action, if possible.
  Redo,
  /// Undoes an action, if possible.
  Undo,
}

pub type BoxedAction = Box<dyn Action>;

/// Implemented by types that define an action that can be redone and undone.
pub trait Action: Send + Sync {
  /// Redoes the action.
  fn redo(&self, world: &mut World) -> ActionResult;
  /// Undoes the action.
  fn undo(&self, world: &mut World) -> ActionResult;
}

/// The result of redoing or undoing an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionResult {
  /// Failed to apply.
  Failed,
  /// Applied successfully.
  Success,
}

/// Consumes all of the [`ActionMessage`]s and applies them to the [`ActionHistory`].
pub fn consume_actions_messages(world: &mut World) {
  world.resource_scope(|world, mut action_history: Mut<ActionHistory>| {
    world.resource_scope(
      |world, mut messages: Mut<Messages<ActionMessage>>| {
        for message in messages.drain() {
          match message {
            ActionMessage::Push(action) => action_history.push(action, world),
            ActionMessage::Redo => action_history.redo(world),
            ActionMessage::Undo => action_history.undo(world),
          }
        }
      },
    );
  });
}

#[derive(Deref, DerefMut)]
pub struct SelectionAction(pub Entity);

impl Action for SelectionAction {
  fn redo(&self, world: &mut World) -> ActionResult {
    let mut entity = world.entity_mut(**self);

    if entity.contains::<Selected>() {
      entity.remove::<Selected>();
    } else {
      entity.insert(Selected);
    }

    ActionResult::Success
  }

  fn undo(&self, world: &mut World) -> ActionResult {
    self.redo(world)
  }
}

#[derive(Deref, DerefMut)]
pub struct CombinedAction(pub Vec<BoxedAction>);

impl FromIterator<BoxedAction> for CombinedAction {
  fn from_iter<T: IntoIterator<Item = BoxedAction>>(iter: T) -> Self {
    Self(iter.into_iter().collect())
  }
}

impl Action for CombinedAction {
  fn redo(&self, world: &mut World) -> ActionResult {
    for (i, action) in self.iter().enumerate() {
      if action.redo(world) == ActionResult::Failed {
        warn!(
          "Could not redo all actions in combined action. Unwinding partial state"
        );

        for action in self.iter().take(i.saturating_sub(1)).rev() {
          if action.undo(world) == ActionResult::Failed {
            warn!(
              "Failed to unwind partial state. There may be more information above"
            );
            return ActionResult::Failed;
          }
        }

        warn!("Successfully unwound partial state");
        return ActionResult::Failed;
      }
    }

    ActionResult::Success
  }

  fn undo(&self, world: &mut World) -> ActionResult {
    for (i, action) in self.iter().enumerate().rev() {
      if action.undo(world) == ActionResult::Failed {
        warn!(
          "Could not undo all actions in combined action. Unwinding partial state"
        );

        for action in self.iter().rev().take(i.saturating_sub(1)).rev() {
          if action.undo(world) == ActionResult::Failed {
            warn!(
              "Failed to unwind partial state. There may be more information above"
            );
            return ActionResult::Failed;
          }
        }

        warn!("Successfully unwound partial state");
        return ActionResult::Failed;
      }
    }

    ActionResult::Success
  }
}
