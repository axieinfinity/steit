using System;

namespace Steit.State {
    public sealed class VariantUpdateEventArgs<TContainer> : EventArgs where TContainer : IEnumState {
        public UInt32 NewTag { get; }
        public IState NewVariant { get; }

        public UInt32 OldTag { get; }
        public IState OldVariant { get; }

        public TContainer Container { get; }

        public VariantUpdateEventArgs(
            UInt32 newTag,
            IState newVariant,
            UInt32 oldTag,
            IState oldVariant,
            TContainer container
        ) {
            this.NewTag = newTag;
            this.NewVariant = newVariant;

            this.OldTag = oldTag;
            this.OldVariant = oldVariant;

            this.Container = container;
        }
    }
}
