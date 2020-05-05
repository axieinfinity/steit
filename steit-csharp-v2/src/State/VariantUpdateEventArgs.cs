using System;

namespace Steit.State {
    public sealed class VariantUpdateEventArgs<TContainer> : EventArgs where TContainer : IEnumState {
        public UInt32 NewVariantTag { get; }
        public IState NewVariant { get; }

        public UInt32 OldVariantTag { get; }
        public IState OldVariant { get; }

        public TContainer Container { get; }

        public VariantUpdateEventArgs(
            UInt32 newVariantTag,
            IState newVariant,
            UInt32 oldVariantTag,
            IState oldVariant,
            TContainer container
        ) {
            this.NewVariantTag = newVariantTag;
            this.NewVariant = newVariant;

            this.OldVariantTag = oldVariantTag;
            this.OldVariant = oldVariant;

            this.Container = container;
        }
    }
}
