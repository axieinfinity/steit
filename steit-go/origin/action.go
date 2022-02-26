package origin

// type ActionEnum uint32

// const (
//     ActionRawTag ActionEnum = 0
//     ActionDamageTag ActionEnum = 1
//     ActionEnergyGainTag ActionEnum = 2
//     ActionEnergyLossTag ActionEnum = 3
//     ActionEnergyTransferTag ActionEnum = 4
//     ActionCardMoveTag ActionEnum = 5
//     ActionPileMoveTag ActionEnum = 6
//     ActionCardDrawTag ActionEnum = 7
//     ActionCardDiscardTag ActionEnum = 8
//     ActionCardBanishTag ActionEnum = 9
//     ActionCardPlayTag ActionEnum = 11
//     ActionStatusRevealTag ActionEnum = 12
//     ActionAttackTag ActionEnum = 13
//     ActionSkillCastTag ActionEnum = 14
//     ActionHealingTag ActionEnum = 15
//     ActionShieldGainTag ActionEnum = 16
//     ActionShieldReductionTag ActionEnum = 17
//     ActionSummoningTag ActionEnum = 18
//     ActionCardAdditionTag ActionEnum = 19
//     ActionCardRevelationTag ActionEnum = 20
//     ActionStatusCastTag ActionEnum = 21
//     ActionPowerCastTag ActionEnum = 22
//     ActionScryTag ActionEnum = 23
//     ActionCurseCastTag ActionEnum = 24
//     ActionStatusAdditionTag ActionEnum = 25
//     ActionBloodMoonCurseTag ActionEnum = 26
//     ActionApplyPoisonTag ActionEnum = 27
//     ActionApplyRageTag ActionEnum = 28
//     ActionChimeraActionTag ActionEnum = 29
//     ActionEndgameTag ActionEnum = 30
//     ActionEndTurnTag ActionEnum = 31
//     ActionUpdateChimeraIntentsTag ActionEnum = 32
//     ActionChangeChimeraWaveTag ActionEnum = 33
//     ActionDeadFighterCardTransformTag ActionEnum = 34
// )

// type Action struct {
//     Path *Path

//     Tag uint32
//     Variant IState
//     OnUpdate *EventHandler
// }

// func (s *Action) RawVariant() ActionRaw {
//     return s.Variant
// }
// func (s *Action) DamageVariant() ActionDamage {
//     return s.Variant
// }
// func (s *Action) EnergyGainVariant() ActionEnergyGain {
//     return s.Variant
// }
// func (s *Action) EnergyLossVariant() ActionEnergyLoss {
//     return s.Variant
// }
// func (s *Action) EnergyTransferVariant() ActionEnergyTransfer {
//     return s.Variant
// }
// func (s *Action) CardMoveVariant() ActionCardMove {
//     return s.Variant
// }
// func (s *Action) PileMoveVariant() ActionPileMove {
//     return s.Variant
// }
// func (s *Action) CardDrawVariant() ActionCardDraw {
//     return s.Variant
// }
// func (s *Action) CardDiscardVariant() ActionCardDiscard {
//     return s.Variant
// }
// func (s *Action) CardBanishVariant() ActionCardBanish {
//     return s.Variant
// }
// func (s *Action) CardPlayVariant() ActionCardPlay {
//     return s.Variant
// }
// func (s *Action) StatusRevealVariant() ActionStatusReveal {
//     return s.Variant
// }
// func (s *Action) AttackVariant() ActionAttack {
//     return s.Variant
// }
// func (s *Action) SkillCastVariant() ActionSkillCast {
//     return s.Variant
// }
// func (s *Action) HealingVariant() ActionHealing {
//     return s.Variant
// }
// func (s *Action) ShieldGainVariant() ActionShieldGain {
//     return s.Variant
// }
// func (s *Action) ShieldReductionVariant() ActionShieldReduction {
//     return s.Variant
// }
// func (s *Action) SummoningVariant() ActionSummoning {
//     return s.Variant
// }
// func (s *Action) CardAdditionVariant() ActionCardAddition {
//     return s.Variant
// }
// func (s *Action) CardRevelationVariant() ActionCardRevelation {
//     return s.Variant
// }
// func (s *Action) StatusCastVariant() ActionStatusCast {
//     return s.Variant
// }
// func (s *Action) PowerCastVariant() ActionPowerCast {
//     return s.Variant
// }
// func (s *Action) ScryVariant() ActionScry {
//     return s.Variant
// }
// func (s *Action) CurseCastVariant() ActionCurseCast {
//     return s.Variant
// }
// func (s *Action) StatusAdditionVariant() ActionStatusAddition {
//     return s.Variant
// }
// func (s *Action) BloodMoonCurseVariant() ActionBloodMoonCurse {
//     return s.Variant
// }
// func (s *Action) ApplyPoisonVariant() ActionApplyPoison {
//     return s.Variant
// }
// func (s *Action) ApplyRageVariant() ActionApplyRage {
//     return s.Variant
// }
// func (s *Action) ChimeraActionVariant() ActionChimeraAction {
//     return s.Variant
// }
// func (s *Action) EndgameVariant() ActionEndgame {
//     return s.Variant
// }
// func (s *Action) EndTurnVariant() ActionEndTurn {
//     return s.Variant
// }
// func (s *Action) UpdateChimeraIntentsVariant() ActionUpdateChimeraIntents {
//     return s.Variant
// }
// func (s *Action) ChangeChimeraWaveVariant() ActionChangeChimeraWave {
//     return s.Variant
// }
// func (s *Action) DeadFighterCardTransformVariant() ActionDeadFighterCardTransform {
//     return s.Variant
// }
// func NewAction(path *Path, tag uint32) Action {
//     obj := Action{Tag: tag}
//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     switch tag {
//         case 0: obj.Variant = NewActionRaw(obj.Path.GetNested(0), 0)
//         case 1: obj.Variant = NewActionDamage(obj.Path.GetNested(1), 0)
//         case 2: obj.Variant = NewActionEnergyGain(obj.Path.GetNested(2), 0)
//         case 3: obj.Variant = NewActionEnergyLoss(obj.Path.GetNested(3), 0)
//         case 4: obj.Variant = NewActionEnergyTransfer(obj.Path.GetNested(4), 0)
//         case 5: obj.Variant = NewActionCardMove(obj.Path.GetNested(5), 0)
//         case 6: obj.Variant = NewActionPileMove(obj.Path.GetNested(6), 0)
//         case 7: obj.Variant = NewActionCardDraw(obj.Path.GetNested(7), 0)
//         case 8: obj.Variant = NewActionCardDiscard(obj.Path.GetNested(8), 0)
//         case 9: obj.Variant = NewActionCardBanish(obj.Path.GetNested(9), 0)
//         case 11: obj.Variant = NewActionCardPlay(obj.Path.GetNested(11), 0)
//         case 12: obj.Variant = NewActionStatusReveal(obj.Path.GetNested(12), 0)
//         case 13: obj.Variant = NewActionAttack(obj.Path.GetNested(13), 0)
//         case 14: obj.Variant = NewActionSkillCast(obj.Path.GetNested(14), 0)
//         case 15: obj.Variant = NewActionHealing(obj.Path.GetNested(15), 0)
//         case 16: obj.Variant = NewActionShieldGain(obj.Path.GetNested(16), 0)
//         case 17: obj.Variant = NewActionShieldReduction(obj.Path.GetNested(17), 0)
//         case 18: obj.Variant = NewActionSummoning(obj.Path.GetNested(18), 0)
//         case 19: obj.Variant = NewActionCardAddition(obj.Path.GetNested(19), 0)
//         case 20: obj.Variant = NewActionCardRevelation(obj.Path.GetNested(20), 0)
//         case 21: obj.Variant = NewActionStatusCast(obj.Path.GetNested(21), 0)
//         case 22: obj.Variant = NewActionPowerCast(obj.Path.GetNested(22), 0)
//         case 23: obj.Variant = NewActionScry(obj.Path.GetNested(23), 0)
//         case 24: obj.Variant = NewActionCurseCast(obj.Path.GetNested(24), 0)
//         case 25: obj.Variant = NewActionStatusAddition(obj.Path.GetNested(25), 0)
//         case 26: obj.Variant = NewActionBloodMoonCurse(obj.Path.GetNested(26), 0)
//         case 27: obj.Variant = NewActionApplyPoison(obj.Path.GetNested(27), 0)
//         case 28: obj.Variant = NewActionApplyRage(obj.Path.GetNested(28), 0)
//         case 29: obj.Variant = NewActionChimeraAction(obj.Path.GetNested(29), 0)
//         case 30: obj.Variant = NewActionEndgame(obj.Path.GetNested(30), 0)
//         case 31: obj.Variant = NewActionEndTurn(obj.Path.GetNested(31), 0)
//         case 32: obj.Variant = NewActionUpdateChimeraIntents(obj.Path.GetNested(32), 0)
//         case 33: obj.Variant = NewActionChangeChimeraWave(obj.Path.GetNested(33), 0)
//         case 34: obj.Variant = NewActionDeadFighterCardTransform(obj.Path.GetNested(34), 0)
//         default: obj.Variant = NewActionRaw(obj.Path.GetNested(0), 0)
//     }
//     return obj
// }

// func (s *Action) NewRaw(path *Path, tag uint32) Action { return NewAction(path, 0) }
// func (s *Action) NewDamage(path *Path, tag uint32) Action { return NewAction(path, 1) }
// func (s *Action) NewEnergyGain(path *Path, tag uint32) Action { return NewAction(path, 2) }
// func (s *Action) NewEnergyLoss(path *Path, tag uint32) Action { return NewAction(path, 3) }
// func (s *Action) NewEnergyTransfer(path *Path, tag uint32) Action { return NewAction(path, 4) }
// func (s *Action) NewCardMove(path *Path, tag uint32) Action { return NewAction(path, 5) }
// func (s *Action) NewPileMove(path *Path, tag uint32) Action { return NewAction(path, 6) }
// func (s *Action) NewCardDraw(path *Path, tag uint32) Action { return NewAction(path, 7) }
// func (s *Action) NewCardDiscard(path *Path, tag uint32) Action { return NewAction(path, 8) }
// func (s *Action) NewCardBanish(path *Path, tag uint32) Action { return NewAction(path, 9) }
// func (s *Action) NewCardPlay(path *Path, tag uint32) Action { return NewAction(path, 11) }
// func (s *Action) NewStatusReveal(path *Path, tag uint32) Action { return NewAction(path, 12) }
// func (s *Action) NewAttack(path *Path, tag uint32) Action { return NewAction(path, 13) }
// func (s *Action) NewSkillCast(path *Path, tag uint32) Action { return NewAction(path, 14) }
// func (s *Action) NewHealing(path *Path, tag uint32) Action { return NewAction(path, 15) }
// func (s *Action) NewShieldGain(path *Path, tag uint32) Action { return NewAction(path, 16) }
// func (s *Action) NewShieldReduction(path *Path, tag uint32) Action { return NewAction(path, 17) }
// func (s *Action) NewSummoning(path *Path, tag uint32) Action { return NewAction(path, 18) }
// func (s *Action) NewCardAddition(path *Path, tag uint32) Action { return NewAction(path, 19) }
// func (s *Action) NewCardRevelation(path *Path, tag uint32) Action { return NewAction(path, 20) }
// func (s *Action) NewStatusCast(path *Path, tag uint32) Action { return NewAction(path, 21) }
// func (s *Action) NewPowerCast(path *Path, tag uint32) Action { return NewAction(path, 22) }
// func (s *Action) NewScry(path *Path, tag uint32) Action { return NewAction(path, 23) }
// func (s *Action) NewCurseCast(path *Path, tag uint32) Action { return NewAction(path, 24) }
// func (s *Action) NewStatusAddition(path *Path, tag uint32) Action { return NewAction(path, 25) }
// func (s *Action) NewBloodMoonCurse(path *Path, tag uint32) Action { return NewAction(path, 26) }
// func (s *Action) NewApplyPoison(path *Path, tag uint32) Action { return NewAction(path, 27) }
// func (s *Action) NewApplyRage(path *Path, tag uint32) Action { return NewAction(path, 28) }
// func (s *Action) NewChimeraAction(path *Path, tag uint32) Action { return NewAction(path, 29) }
// func (s *Action) NewEndgame(path *Path, tag uint32) Action { return NewAction(path, 30) }
// func (s *Action) NewEndTurn(path *Path, tag uint32) Action { return NewAction(path, 31) }
// func (s *Action) NewUpdateChimeraIntents(path *Path, tag uint32) Action { return NewAction(path, 32) }
// func (s *Action) NewChangeChimeraWave(path *Path, tag uint32) Action { return NewAction(path, 33) }
// func (s *Action) NewDeadFighterCardTransform(path *Path, tag uint32) Action { return NewAction(path, 34) }

// func (s *Action) ClearUpdateHandlers() {
//     s.OnUpdate = nil
// }

// func ActionDeserialize(_type reflect.Type, reader IReader, path *Path) Action {
//     action := NewAction(path, 0)
//     action.Replace(reader, false)
//     return action
// }

// func (s *Action) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Sized
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Sized
//     case 5: return &WireType.Sized
//     case 6: return &WireType.Sized
//     case 7: return &WireType.Sized
//     case 8: return &WireType.Sized
//     case 9: return &WireType.Sized
//     case 11: return &WireType.Sized
//     case 12: return &WireType.Sized
//     case 13: return &WireType.Sized
//     case 14: return &WireType.Sized
//     case 15: return &WireType.Sized
//     case 16: return &WireType.Sized
//     case 17: return &WireType.Sized
//     case 18: return &WireType.Sized
//     case 19: return &WireType.Sized
//     case 20: return &WireType.Sized
//     case 21: return &WireType.Sized
//     case 22: return &WireType.Sized
//     case 23: return &WireType.Sized
//     case 24: return &WireType.Sized
//     case 25: return &WireType.Sized
//     case 26: return &WireType.Sized
//     case 27: return &WireType.Sized
//     case 28: return &WireType.Sized
//     case 29: return &WireType.Sized
//     case 30: return &WireType.Sized
//     case 31: return &WireType.Sized
//     case 32: return &WireType.Sized
//     case 33: return &WireType.Sized
//     case 34: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *Action) GetNested(tag uint32) *IState {
//     if tag == s.Tag {
//         return &s.Variant
//     } else {
//         return nil
//     }
// }

// func (s *Action) ReplaceAt(tag uint32, wireType WireType, reader IReader, shouldNotify bool) {
//     switch tag {
//     case 0: s.UpdateAndNotify(0, ActionRawDeserialize(reader, s.Path.GetNested(0)), shouldNotify)
//     case 1: s.UpdateAndNotify(1, ActionDamageDeserialize(reader, s.Path.GetNested(1)), shouldNotify)
//     case 2: s.UpdateAndNotify(2, ActionEnergyGainDeserialize(reader, s.Path.GetNested(2)), shouldNotify)
//     case 3: s.UpdateAndNotify(3, ActionEnergyLossDeserialize(reader, s.Path.GetNested(3)), shouldNotify)
//     case 4: s.UpdateAndNotify(4, ActionEnergyTransferDeserialize(reader, s.Path.GetNested(4)), shouldNotify)
//     case 5: s.UpdateAndNotify(5, ActionCardMoveDeserialize(reader, s.Path.GetNested(5)), shouldNotify)
//     case 6: s.UpdateAndNotify(6, ActionPileMoveDeserialize(reader, s.Path.GetNested(6)), shouldNotify)
//     case 7: s.UpdateAndNotify(7, ActionCardDrawDeserialize(reader, s.Path.GetNested(7)), shouldNotify)
//     case 8: s.UpdateAndNotify(8, ActionCardDiscardDeserialize(reader, s.Path.GetNested(8)), shouldNotify)
//     case 9: s.UpdateAndNotify(9, ActionCardBanishDeserialize(reader, s.Path.GetNested(9)), shouldNotify)
//     case 11: s.UpdateAndNotify(11, ActionCardPlayDeserialize(reader, s.Path.GetNested(11)), shouldNotify)
//     case 12: s.UpdateAndNotify(12, ActionStatusRevealDeserialize(reader, s.Path.GetNested(12)), shouldNotify)
//     case 13: s.UpdateAndNotify(13, ActionAttackDeserialize(reader, s.Path.GetNested(13)), shouldNotify)
//     case 14: s.UpdateAndNotify(14, ActionSkillCastDeserialize(reader, s.Path.GetNested(14)), shouldNotify)
//     case 15: s.UpdateAndNotify(15, ActionHealingDeserialize(reader, s.Path.GetNested(15)), shouldNotify)
//     case 16: s.UpdateAndNotify(16, ActionShieldGainDeserialize(reader, s.Path.GetNested(16)), shouldNotify)
//     case 17: s.UpdateAndNotify(17, ActionShieldReductionDeserialize(reader, s.Path.GetNested(17)), shouldNotify)
//     case 18: s.UpdateAndNotify(18, ActionSummoningDeserialize(reader, s.Path.GetNested(18)), shouldNotify)
//     case 19: s.UpdateAndNotify(19, ActionCardAdditionDeserialize(reader, s.Path.GetNested(19)), shouldNotify)
//     case 20: s.UpdateAndNotify(20, ActionCardRevelationDeserialize(reader, s.Path.GetNested(20)), shouldNotify)
//     case 21: s.UpdateAndNotify(21, ActionStatusCastDeserialize(reader, s.Path.GetNested(21)), shouldNotify)
//     case 22: s.UpdateAndNotify(22, ActionPowerCastDeserialize(reader, s.Path.GetNested(22)), shouldNotify)
//     case 23: s.UpdateAndNotify(23, ActionScryDeserialize(reader, s.Path.GetNested(23)), shouldNotify)
//     case 24: s.UpdateAndNotify(24, ActionCurseCastDeserialize(reader, s.Path.GetNested(24)), shouldNotify)
//     case 25: s.UpdateAndNotify(25, ActionStatusAdditionDeserialize(reader, s.Path.GetNested(25)), shouldNotify)
//     case 26: s.UpdateAndNotify(26, ActionBloodMoonCurseDeserialize(reader, s.Path.GetNested(26)), shouldNotify)
//     case 27: s.UpdateAndNotify(27, ActionApplyPoisonDeserialize(reader, s.Path.GetNested(27)), shouldNotify)
//     case 28: s.UpdateAndNotify(28, ActionApplyRageDeserialize(reader, s.Path.GetNested(28)), shouldNotify)
//     case 29: s.UpdateAndNotify(29, ActionChimeraActionDeserialize(reader, s.Path.GetNested(29)), shouldNotify)
//     case 30: s.UpdateAndNotify(30, ActionEndgameDeserialize(reader, s.Path.GetNested(30)), shouldNotify)
//     case 31: s.UpdateAndNotify(31, ActionEndTurnDeserialize(reader, s.Path.GetNested(31)), shouldNotify)
//     case 32: s.UpdateAndNotify(32, ActionUpdateChimeraIntentsDeserialize(reader, s.Path.GetNested(32)), shouldNotify)
//     case 33: s.UpdateAndNotify(33, ActionChangeChimeraWaveDeserialize(reader, s.Path.GetNested(33)), shouldNotify)
//     case 34: s.UpdateAndNotify(34, ActionDeadFighterCardTransformDeserialize(reader, s.Path.GetNested(34)), shouldNotify)
//     default: reader.SkipToEnd()
//     }
// }

// func (s *Action) ReplayListPush(IReader reader) { panic("") }
// func (s *Action) ReplayListPop(IReader reader) { panic("") }
// func (s *Action) ReplayMapRemove(IReader reader) { panic("") }

// func (s *Action) UpdateAndNotify(newTag uint32, newVariant IState, shouldNotify bool) {
//     if shouldNotify {
//         args := NewVariantUpdateEventArgs(newTag, newVariant, s.Tag, s.Variant, s)
//         Action.OnUpdate.Invoke(this, args)
//     }

//     s.Tag = newTag
//     s.Variant = newVariant
// }

// // Variant (0): Raw
// type ActionRaw struct {
//     Path *Path
//     OnEntriesUpdate *EventHandler
//     Entries Vector
// }

// func NewActionRaw(path *Path, tag uint32) ActionRaw {
//     obj := ActionRaw{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.Entries = NewVector(obj.Path.GetNested(0), 0)
//     return obj
// }

// func (s *ActionRaw) ClearEntriesUpdateHandlers() {
//     s.OnEntriesUpdate = nil
// }

// func (s *ActionRaw) ClearUpdateHandlers() {
//     s.OnEntriesUpdate = nil}

// func ActionRawDeserialize(_type reflect.Type, reader IReader, path *Path) ActionRaw {
//     raw := NewActionRaw(path, 0)
//     raw.Replace(reader, false)
//     return raw
// }

// func (s *ActionRaw) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionRaw) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.Entries
//     default: return nil
//     }
// }

// func (s *ActionRaw) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.Entries = s.MaybeNotify(0, VectorDeserialize(reader, s.Path.GetNested(0)), s.Entries, s.OnEntriesUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionRaw) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionRaw) ReplayListPop() { panic("") }
// func (s *ActionRaw) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionRaw) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (1): Damage
// type ActionDamage struct {
//     Path *Path
//     OnSourceFighterIndexUpdate *EventHandler
//     OnSourceCardIdUpdate *EventHandler
//     OnSourceStatusIdUpdate *EventHandler
//     OnSourceChimeraAbilityUpdate *EventHandler
//     OnTargetFighterIndexUpdate *EventHandler
//     OnDamageTypeUpdate *EventHandler
//     OnBlockDecisionUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnCritDecisionUpdate *EventHandler
//     OnIsCritUpdate *EventHandler
//     OnCritBlockDecisionUpdate *EventHandler
//     OnIsCritBlockedUpdate *EventHandler
//     OnBeforeDamageUpdate *EventHandler
//     OnDamageUpdate *EventHandler
//     OnOutputDamageUpdate *EventHandler
//     OnAfterDamageUpdate *EventHandler
//     SourceFighterIndex Option
//     SourceCardId Option
//     SourceStatusId Option
//     SourceChimeraAbility Option
//     TargetFighterIndex uint32
//     DamageType DamageType
//     BlockDecision Vector
//     IsBlocked bool
//     CritDecision Vector
//     IsCrit bool
//     CritBlockDecision Vector
//     IsCritBlocked bool
//     BeforeDamage Vector
//     Damage Vector
//     OutputDamage uint32
//     AfterDamage Vector
// }

// func NewActionDamage(path *Path, tag uint32) ActionDamage {
//     obj := ActionDamage{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.SourceFighterIndex = NewOption(obj.Path.GetNested(0), 0)
//     obj.SourceCardId = NewOption(obj.Path.GetNested(1), 0)
//     obj.SourceStatusId = NewOption(obj.Path.GetNested(2), 0)
//     obj.SourceChimeraAbility = NewOption(obj.Path.GetNested(15), 0)
//     obj.DamageType = NewDamageType(obj.Path.GetNested(4), 0)
//     obj.BlockDecision = NewVector(obj.Path.GetNested(5), 0)
//     obj.CritDecision = NewVector(obj.Path.GetNested(7), 0)
//     obj.CritBlockDecision = NewVector(obj.Path.GetNested(9), 0)
//     obj.BeforeDamage = NewVector(obj.Path.GetNested(11), 0)
//     obj.Damage = NewVector(obj.Path.GetNested(12), 0)
//     obj.AfterDamage = NewVector(obj.Path.GetNested(14), 0)
//     return obj
// }

// func (s *ActionDamage) ClearSourceFighterIndexUpdateHandlers() { s.OnSourceFighterIndexUpdate = nil }
// func (s *ActionDamage) ClearSourceCardIdUpdateHandlers() { s.OnSourceCardIdUpdate = nil }
// func (s *ActionDamage) ClearSourceStatusIdUpdateHandlers() { s.OnSourceStatusIdUpdate = nil }
// func (s *ActionDamage) ClearSourceChimeraAbilityUpdateHandlers() { s.OnSourceChimeraAbilityUpdate = nil }
// func (s *ActionDamage) ClearTargetFighterIndexUpdateHandlers() { s.OnTargetFighterIndexUpdate = nil }
// func (s *ActionDamage) ClearDamageTypeUpdateHandlers() { s.OnDamageTypeUpdate = nil }
// func (s *ActionDamage) ClearBlockDecisionUpdateHandlers() { s.OnBlockDecisionUpdate = nil }
// func (s *ActionDamage) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionDamage) ClearCritDecisionUpdateHandlers() { s.OnCritDecisionUpdate = nil }
// func (s *ActionDamage) ClearIsCritUpdateHandlers() { s.OnIsCritUpdate = nil }
// func (s *ActionDamage) ClearCritBlockDecisionUpdateHandlers() { s.OnCritBlockDecisionUpdate = nil }
// func (s *ActionDamage) ClearIsCritBlockedUpdateHandlers() { s.OnIsCritBlockedUpdate = nil }
// func (s *ActionDamage) ClearBeforeDamageUpdateHandlers() { s.OnBeforeDamageUpdate = nil }
// func (s *ActionDamage) ClearDamageUpdateHandlers() { s.OnDamageUpdate = nil }
// func (s *ActionDamage) ClearOutputDamageUpdateHandlers() { s.OnOutputDamageUpdate = nil }
// func (s *ActionDamage) ClearAfterDamageUpdateHandlers() { s.OnAfterDamageUpdate = nil }

// func (s *ActionDamage) ClearUpdateHandlers() {
//     s.OnSourceFighterIndexUpdate = nil
//     s.OnSourceCardIdUpdate = nil
//     s.OnSourceStatusIdUpdate = nil
//     s.OnSourceChimeraAbilityUpdate = nil
//     s.OnTargetFighterIndexUpdate = nil
//     s.OnDamageTypeUpdate = nil
//     s.OnBlockDecisionUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnCritDecisionUpdate = nil
//     s.OnIsCritUpdate = nil
//     s.OnCritBlockDecisionUpdate = nil
//     s.OnIsCritBlockedUpdate = nil
//     s.OnBeforeDamageUpdate = nil
//     s.OnDamageUpdate = nil
//     s.OnOutputDamageUpdate = nil
//     s.OnAfterDamageUpdate = nil}

// func ActionDamageDeserialize(_type reflect.Type, reader IReader, path *Path) ActionDamage {
//     damage := NewActionDamage(path, 0)
//     damage.Replace(reader, false)
//     return damage
// }

// func (s *ActionDamage) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Sized
//     case 15: return &WireType.Sized
//     case 3: return &WireType.Varint
//     case 4: return &WireType.Sized
//     case 5: return &WireType.Sized
//     case 6: return &WireType.Varint
//     case 7: return &WireType.Sized
//     case 8: return &WireType.Varint
//     case 9: return &WireType.Sized
//     case 10: return &WireType.Varint
//     case 11: return &WireType.Sized
//     case 12: return &WireType.Sized
//     case 13: return &WireType.Varint
//     case 14: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionDamage) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.SourceFighterIndex
//     case 1: return &s.SourceCardId
//     case 2: return &s.SourceStatusId
//     case 15: return &s.SourceChimeraAbility
//     case 4: return &s.DamageType
//     case 5: return &s.BlockDecision
//     case 7: return &s.CritDecision
//     case 9: return &s.CritBlockDecision
//     case 11: return &s.BeforeDamage
//     case 12: return &s.Damage
//     case 14: return &s.AfterDamage
//     default: return nil
//     }
// }

// func (s *ActionDamage) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.SourceFighterIndex = s.MaybeNotify(0, OptionDeserialize(reader, s.Path.GetNested(0)), s.SourceFighterIndex, s.OnSourceFighterIndexUpdate, shouldNotify)
//     case 1: s.SourceCardId = s.MaybeNotify(1, OptionDeserialize(reader, s.Path.GetNested(1)), s.SourceCardId, s.OnSourceCardIdUpdate, shouldNotify)
//     case 2: s.SourceStatusId = s.MaybeNotify(2, OptionDeserialize(reader, s.Path.GetNested(2)), s.SourceStatusId, s.OnSourceStatusIdUpdate, shouldNotify)
//     case 15: s.SourceChimeraAbility = s.MaybeNotify(15, OptionDeserialize(reader, s.Path.GetNested(15)), s.SourceChimeraAbility, s.OnSourceChimeraAbilityUpdate, shouldNotify)
//     case 3: s.TargetFighterIndex = s.MaybeNotify(3, reader.Readuint32(), s.TargetFighterIndex, s.OnTargetFighterIndexUpdate, shouldNotify)
//     case 4: s.DamageType = s.MaybeNotify(4, DamageTypeDeserialize(reader, s.Path.GetNested(4)), s.DamageType, s.OnDamageTypeUpdate, shouldNotify)
//     case 5: s.BlockDecision = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.BlockDecision, s.OnBlockDecisionUpdate, shouldNotify)
//     case 6: s.IsBlocked = s.MaybeNotify(6, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 7: s.CritDecision = s.MaybeNotify(7, VectorDeserialize(reader, s.Path.GetNested(7)), s.CritDecision, s.OnCritDecisionUpdate, shouldNotify)
//     case 8: s.IsCrit = s.MaybeNotify(8, reader.Readbool(), s.IsCrit, s.OnIsCritUpdate, shouldNotify)
//     case 9: s.CritBlockDecision = s.MaybeNotify(9, VectorDeserialize(reader, s.Path.GetNested(9)), s.CritBlockDecision, s.OnCritBlockDecisionUpdate, shouldNotify)
//     case 10: s.IsCritBlocked = s.MaybeNotify(10, reader.Readbool(), s.IsCritBlocked, s.OnIsCritBlockedUpdate, shouldNotify)
//     case 11: s.BeforeDamage = s.MaybeNotify(11, VectorDeserialize(reader, s.Path.GetNested(11)), s.BeforeDamage, s.OnBeforeDamageUpdate, shouldNotify)
//     case 12: s.Damage = s.MaybeNotify(12, VectorDeserialize(reader, s.Path.GetNested(12)), s.Damage, s.OnDamageUpdate, shouldNotify)
//     case 13: s.OutputDamage = s.MaybeNotify(13, reader.Readuint32(), s.OutputDamage, s.OnOutputDamageUpdate, shouldNotify)
//     case 14: s.AfterDamage = s.MaybeNotify(14, VectorDeserialize(reader, s.Path.GetNested(14)), s.AfterDamage, s.OnAfterDamageUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionDamage) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionDamage) ReplayListPop() { panic("") }
// func (s *ActionDamage) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionDamage) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (2): EnergyGain
// type ActionEnergyGain struct {
//     Path *Path
//     OnPlayerIndexUpdate *EventHandler
//     OnBeforeGainUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnGainUpdate *EventHandler
//     OnEnergyAmountUpdate *EventHandler
//     OnAfterGainUpdate *EventHandler
//     PlayerIndex uint32
//     BeforeGain Vector
//     IsBlocked bool
//     Gain Vector
//     EnergyAmount uint8
//     AfterGain Vector
// }

// func NewActionEnergyGain(path *Path, tag uint32) ActionEnergyGain {
//     obj := ActionEnergyGain{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.BeforeGain = NewVector(obj.Path.GetNested(1), 0)
//     obj.Gain = NewVector(obj.Path.GetNested(3), 0)
//     obj.AfterGain = NewVector(obj.Path.GetNested(5), 0)
//     return obj
// }

// func (s *ActionEnergyGain) ClearPlayerIndexUpdateHandlers() { s.OnPlayerIndexUpdate = nil }
// func (s *ActionEnergyGain) ClearBeforeGainUpdateHandlers() { s.OnBeforeGainUpdate = nil }
// func (s *ActionEnergyGain) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionEnergyGain) ClearGainUpdateHandlers() { s.OnGainUpdate = nil }
// func (s *ActionEnergyGain) ClearEnergyAmountUpdateHandlers() { s.OnEnergyAmountUpdate = nil }
// func (s *ActionEnergyGain) ClearAfterGainUpdateHandlers() { s.OnAfterGainUpdate = nil }

// func (s *ActionEnergyGain) ClearUpdateHandlers() {
//     s.OnPlayerIndexUpdate = nil
//     s.OnBeforeGainUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnGainUpdate = nil
//     s.OnEnergyAmountUpdate = nil
//     s.OnAfterGainUpdate = nil}

// func ActionEnergyGainDeserialize(_type reflect.Type, reader IReader, path *Path) ActionEnergyGain {
//     energyGain := NewActionEnergyGain(path, 0)
//     energyGain.Replace(reader, false)
//     return energyGain
// }

// func (s *ActionEnergyGain) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Varint
//     case 5: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionEnergyGain) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.BeforeGain
//     case 3: return &s.Gain
//     case 5: return &s.AfterGain
//     default: return nil
//     }
// }

// func (s *ActionEnergyGain) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.PlayerIndex = s.MaybeNotify(0, reader.Readuint32(), s.PlayerIndex, s.OnPlayerIndexUpdate, shouldNotify)
//     case 1: s.BeforeGain = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.BeforeGain, s.OnBeforeGainUpdate, shouldNotify)
//     case 2: s.IsBlocked = s.MaybeNotify(2, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 3: s.Gain = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.Gain, s.OnGainUpdate, shouldNotify)
//     case 4: s.EnergyAmount = s.MaybeNotify(4, reader.Readuint8(), s.EnergyAmount, s.OnEnergyAmountUpdate, shouldNotify)
//     case 5: s.AfterGain = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.AfterGain, s.OnAfterGainUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionEnergyGain) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionEnergyGain) ReplayListPop() { panic("") }
// func (s *ActionEnergyGain) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionEnergyGain) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (3): EnergyLoss
// type ActionEnergyLoss struct {
//     Path *Path
//     OnPlayerIndexUpdate *EventHandler
//     OnBeforeLossUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnLossUpdate *EventHandler
//     OnEnergyAmountUpdate *EventHandler
//     OnAfterLossUpdate *EventHandler
//     PlayerIndex uint32
//     BeforeLoss Vector
//     IsBlocked bool
//     Loss Vector
//     EnergyAmount uint8
//     AfterLoss Vector
// }

// func NewActionEnergyLoss(path *Path, tag uint32) ActionEnergyLoss {
//     obj := ActionEnergyLoss{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.BeforeLoss = NewVector(obj.Path.GetNested(1), 0)
//     obj.Loss = NewVector(obj.Path.GetNested(3), 0)
//     obj.AfterLoss = NewVector(obj.Path.GetNested(5), 0)
//     return obj
// }

// func (s *ActionEnergyLoss) ClearPlayerIndexUpdateHandlers() { s.OnPlayerIndexUpdate = nil }
// func (s *ActionEnergyLoss) ClearBeforeLossUpdateHandlers() { s.OnBeforeLossUpdate = nil }
// func (s *ActionEnergyLoss) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionEnergyLoss) ClearLossUpdateHandlers() { s.OnLossUpdate = nil }
// func (s *ActionEnergyLoss) ClearEnergyAmountUpdateHandlers() { s.OnEnergyAmountUpdate = nil }
// func (s *ActionEnergyLoss) ClearAfterLossUpdateHandlers() { s.OnAfterLossUpdate = nil }

// func (s *ActionEnergyLoss) ClearUpdateHandlers() {
//     s.OnPlayerIndexUpdate = nil
//     s.OnBeforeLossUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnLossUpdate = nil
//     s.OnEnergyAmountUpdate = nil
//     s.OnAfterLossUpdate = nil}

// func ActionEnergyLossDeserialize(_type reflect.Type, reader IReader, path *Path) ActionEnergyLoss {
//     energyLoss := NewActionEnergyLoss(path, 0)
//     energyLoss.Replace(reader, false)
//     return energyLoss
// }

// func (s *ActionEnergyLoss) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Varint
//     case 5: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionEnergyLoss) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.BeforeLoss
//     case 3: return &s.Loss
//     case 5: return &s.AfterLoss
//     default: return nil
//     }
// }

// func (s *ActionEnergyLoss) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.PlayerIndex = s.MaybeNotify(0, reader.Readuint32(), s.PlayerIndex, s.OnPlayerIndexUpdate, shouldNotify)
//     case 1: s.BeforeLoss = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.BeforeLoss, s.OnBeforeLossUpdate, shouldNotify)
//     case 2: s.IsBlocked = s.MaybeNotify(2, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 3: s.Loss = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.Loss, s.OnLossUpdate, shouldNotify)
//     case 4: s.EnergyAmount = s.MaybeNotify(4, reader.Readuint8(), s.EnergyAmount, s.OnEnergyAmountUpdate, shouldNotify)
//     case 5: s.AfterLoss = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.AfterLoss, s.OnAfterLossUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionEnergyLoss) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionEnergyLoss) ReplayListPop() { panic("") }
// func (s *ActionEnergyLoss) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionEnergyLoss) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (4): EnergyTransfer
// type ActionEnergyTransfer struct {
//     Path *Path
//     OnFromPlayerIndexUpdate *EventHandler
//     OnToPlayerIndexUpdate *EventHandler
//     OnBeforeTransferUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnTransferUpdate *EventHandler
//     OnEnergyAmountUpdate *EventHandler
//     OnAfterTransferUpdate *EventHandler
//     FromPlayerIndex uint32
//     ToPlayerIndex uint32
//     BeforeTransfer Vector
//     IsBlocked bool
//     Transfer Vector
//     EnergyAmount uint8
//     AfterTransfer Vector
// }

// func NewActionEnergyTransfer(path *Path, tag uint32) ActionEnergyTransfer {
//     obj := ActionEnergyTransfer{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.BeforeTransfer = NewVector(obj.Path.GetNested(2), 0)
//     obj.Transfer = NewVector(obj.Path.GetNested(4), 0)
//     obj.AfterTransfer = NewVector(obj.Path.GetNested(6), 0)
//     return obj
// }

// func (s *ActionEnergyTransfer) ClearFromPlayerIndexUpdateHandlers() { s.OnFromPlayerIndexUpdate = nil }
// func (s *ActionEnergyTransfer) ClearToPlayerIndexUpdateHandlers() { s.OnToPlayerIndexUpdate = nil }
// func (s *ActionEnergyTransfer) ClearBeforeTransferUpdateHandlers() { s.OnBeforeTransferUpdate = nil }
// func (s *ActionEnergyTransfer) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionEnergyTransfer) ClearTransferUpdateHandlers() { s.OnTransferUpdate = nil }
// func (s *ActionEnergyTransfer) ClearEnergyAmountUpdateHandlers() { s.OnEnergyAmountUpdate = nil }
// func (s *ActionEnergyTransfer) ClearAfterTransferUpdateHandlers() { s.OnAfterTransferUpdate = nil }

// func (s *ActionEnergyTransfer) ClearUpdateHandlers() {
//     s.OnFromPlayerIndexUpdate = nil
//     s.OnToPlayerIndexUpdate = nil
//     s.OnBeforeTransferUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnTransferUpdate = nil
//     s.OnEnergyAmountUpdate = nil
//     s.OnAfterTransferUpdate = nil}

// func ActionEnergyTransferDeserialize(_type reflect.Type, reader IReader, path *Path) ActionEnergyTransfer {
//     energyTransfer := NewActionEnergyTransfer(path, 0)
//     energyTransfer.Replace(reader, false)
//     return energyTransfer
// }

// func (s *ActionEnergyTransfer) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Varint
//     case 2: return &WireType.Sized
//     case 3: return &WireType.Varint
//     case 4: return &WireType.Sized
//     case 5: return &WireType.Varint
//     case 6: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionEnergyTransfer) GetNested(tag uint32) *IState {
//     switch tag {
//     case 2: return &s.BeforeTransfer
//     case 4: return &s.Transfer
//     case 6: return &s.AfterTransfer
//     default: return nil
//     }
// }

// func (s *ActionEnergyTransfer) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.FromPlayerIndex = s.MaybeNotify(0, reader.Readuint32(), s.FromPlayerIndex, s.OnFromPlayerIndexUpdate, shouldNotify)
//     case 1: s.ToPlayerIndex = s.MaybeNotify(1, reader.Readuint32(), s.ToPlayerIndex, s.OnToPlayerIndexUpdate, shouldNotify)
//     case 2: s.BeforeTransfer = s.MaybeNotify(2, VectorDeserialize(reader, s.Path.GetNested(2)), s.BeforeTransfer, s.OnBeforeTransferUpdate, shouldNotify)
//     case 3: s.IsBlocked = s.MaybeNotify(3, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 4: s.Transfer = s.MaybeNotify(4, VectorDeserialize(reader, s.Path.GetNested(4)), s.Transfer, s.OnTransferUpdate, shouldNotify)
//     case 5: s.EnergyAmount = s.MaybeNotify(5, reader.Readuint8(), s.EnergyAmount, s.OnEnergyAmountUpdate, shouldNotify)
//     case 6: s.AfterTransfer = s.MaybeNotify(6, VectorDeserialize(reader, s.Path.GetNested(6)), s.AfterTransfer, s.OnAfterTransferUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionEnergyTransfer) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionEnergyTransfer) ReplayListPop() { panic("") }
// func (s *ActionEnergyTransfer) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionEnergyTransfer) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (5): CardMove
// type ActionCardMove struct {
//     Path *Path
//     OnSourcePileUpdate *EventHandler
//     OnTargetPileUpdate *EventHandler
//     OnCardIdUpdate *EventHandler
//     OnMoveUpdate *EventHandler
//     SourcePile VisualCardPile
//     TargetPile VisualCardPile
//     CardId uint32
//     Move Vector
// }

// func NewActionCardMove(path *Path, tag uint32) ActionCardMove {
//     obj := ActionCardMove{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.SourcePile = NewVisualCardPile(obj.Path.GetNested(0), 0)
//     obj.TargetPile = NewVisualCardPile(obj.Path.GetNested(1), 0)
//     obj.Move = NewVector(obj.Path.GetNested(3), 0)
//     return obj
// }

// func (s *ActionCardMove) ClearSourcePileUpdateHandlers() { s.OnSourcePileUpdate = nil }
// func (s *ActionCardMove) ClearTargetPileUpdateHandlers() { s.OnTargetPileUpdate = nil }
// func (s *ActionCardMove) ClearCardIdUpdateHandlers() { s.OnCardIdUpdate = nil }
// func (s *ActionCardMove) ClearMoveUpdateHandlers() { s.OnMoveUpdate = nil }

// func (s *ActionCardMove) ClearUpdateHandlers() {
//     s.OnSourcePileUpdate = nil
//     s.OnTargetPileUpdate = nil
//     s.OnCardIdUpdate = nil
//     s.OnMoveUpdate = nil}

// func ActionCardMoveDeserialize(_type reflect.Type, reader IReader, path *Path) ActionCardMove {
//     cardMove := NewActionCardMove(path, 0)
//     cardMove.Replace(reader, false)
//     return cardMove
// }

// func (s *ActionCardMove) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionCardMove) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.SourcePile
//     case 1: return &s.TargetPile
//     case 3: return &s.Move
//     default: return nil
//     }
// }

// func (s *ActionCardMove) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.SourcePile = s.MaybeNotify(0, VisualCardPileDeserialize(reader, s.Path.GetNested(0)), s.SourcePile, s.OnSourcePileUpdate, shouldNotify)
//     case 1: s.TargetPile = s.MaybeNotify(1, VisualCardPileDeserialize(reader, s.Path.GetNested(1)), s.TargetPile, s.OnTargetPileUpdate, shouldNotify)
//     case 2: s.CardId = s.MaybeNotify(2, reader.Readuint32(), s.CardId, s.OnCardIdUpdate, shouldNotify)
//     case 3: s.Move = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.Move, s.OnMoveUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionCardMove) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionCardMove) ReplayListPop() { panic("") }
// func (s *ActionCardMove) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionCardMove) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (6): PileMove
// type ActionPileMove struct {
//     Path *Path
//     OnSourcePileUpdate *EventHandler
//     OnTargetPileUpdate *EventHandler
//     OnNumCardsUpdate *EventHandler
//     OnMoveUpdate *EventHandler
//     SourcePile VisualCardPile
//     TargetPile VisualCardPile
//     NumCards uint32
//     Move Vector
// }

// func NewActionPileMove(path *Path, tag uint32) ActionPileMove {
//     obj := ActionPileMove{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.SourcePile = NewVisualCardPile(obj.Path.GetNested(0), 0)
//     obj.TargetPile = NewVisualCardPile(obj.Path.GetNested(1), 0)
//     obj.Move = NewVector(obj.Path.GetNested(3), 0)
//     return obj
// }

// func (s *ActionPileMove) ClearSourcePileUpdateHandlers() { s.OnSourcePileUpdate = nil }
// func (s *ActionPileMove) ClearTargetPileUpdateHandlers() { s.OnTargetPileUpdate = nil }
// func (s *ActionPileMove) ClearNumCardsUpdateHandlers() { s.OnNumCardsUpdate = nil }
// func (s *ActionPileMove) ClearMoveUpdateHandlers() { s.OnMoveUpdate = nil }

// func (s *ActionPileMove) ClearUpdateHandlers() {
//     s.OnSourcePileUpdate = nil
//     s.OnTargetPileUpdate = nil
//     s.OnNumCardsUpdate = nil
//     s.OnMoveUpdate = nil}

// func ActionPileMoveDeserialize(_type reflect.Type, reader IReader, path *Path) ActionPileMove {
//     pileMove := NewActionPileMove(path, 0)
//     pileMove.Replace(reader, false)
//     return pileMove
// }

// func (s *ActionPileMove) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionPileMove) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.SourcePile
//     case 1: return &s.TargetPile
//     case 3: return &s.Move
//     default: return nil
//     }
// }

// func (s *ActionPileMove) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.SourcePile = s.MaybeNotify(0, VisualCardPileDeserialize(reader, s.Path.GetNested(0)), s.SourcePile, s.OnSourcePileUpdate, shouldNotify)
//     case 1: s.TargetPile = s.MaybeNotify(1, VisualCardPileDeserialize(reader, s.Path.GetNested(1)), s.TargetPile, s.OnTargetPileUpdate, shouldNotify)
//     case 2: s.NumCards = s.MaybeNotify(2, reader.Readuint32(), s.NumCards, s.OnNumCardsUpdate, shouldNotify)
//     case 3: s.Move = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.Move, s.OnMoveUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionPileMove) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionPileMove) ReplayListPop() { panic("") }
// func (s *ActionPileMove) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionPileMove) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (7): CardDraw
// type ActionCardDraw struct {
//     Path *Path
//     OnPlayerIndexUpdate *EventHandler
//     OnFullCardsInHandUpdate *EventHandler
//     OnReshuffleUpdate *EventHandler
//     OnNoCardsLeftUpdate *EventHandler
//     OnBeforeDrawUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnDrawUpdate *EventHandler
//     OnCardIdUpdate *EventHandler
//     OnAfterDrawUpdate *EventHandler
//     PlayerIndex uint32
//     FullCardsInHand bool
//     Reshuffle Vector
//     NoCardsLeft bool
//     BeforeDraw Vector
//     IsBlocked bool
//     Draw Vector
//     CardId Option
//     AfterDraw Vector
// }

// func NewActionCardDraw(path *Path, tag uint32) ActionCardDraw {
//     obj := ActionCardDraw{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.Reshuffle = NewVector(obj.Path.GetNested(1), 0)
//     obj.BeforeDraw = NewVector(obj.Path.GetNested(3), 0)
//     obj.Draw = NewVector(obj.Path.GetNested(5), 0)
//     obj.CardId = NewOption(obj.Path.GetNested(6), 0)
//     obj.AfterDraw = NewVector(obj.Path.GetNested(7), 0)
//     return obj
// }

// func (s *ActionCardDraw) ClearPlayerIndexUpdateHandlers() { s.OnPlayerIndexUpdate = nil }
// func (s *ActionCardDraw) ClearFullCardsInHandUpdateHandlers() { s.OnFullCardsInHandUpdate = nil }
// func (s *ActionCardDraw) ClearReshuffleUpdateHandlers() { s.OnReshuffleUpdate = nil }
// func (s *ActionCardDraw) ClearNoCardsLeftUpdateHandlers() { s.OnNoCardsLeftUpdate = nil }
// func (s *ActionCardDraw) ClearBeforeDrawUpdateHandlers() { s.OnBeforeDrawUpdate = nil }
// func (s *ActionCardDraw) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionCardDraw) ClearDrawUpdateHandlers() { s.OnDrawUpdate = nil }
// func (s *ActionCardDraw) ClearCardIdUpdateHandlers() { s.OnCardIdUpdate = nil }
// func (s *ActionCardDraw) ClearAfterDrawUpdateHandlers() { s.OnAfterDrawUpdate = nil }

// func (s *ActionCardDraw) ClearUpdateHandlers() {
//     s.OnPlayerIndexUpdate = nil
//     s.OnFullCardsInHandUpdate = nil
//     s.OnReshuffleUpdate = nil
//     s.OnNoCardsLeftUpdate = nil
//     s.OnBeforeDrawUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnDrawUpdate = nil
//     s.OnCardIdUpdate = nil
//     s.OnAfterDrawUpdate = nil}

// func ActionCardDrawDeserialize(_type reflect.Type, reader IReader, path *Path) ActionCardDraw {
//     cardDraw := NewActionCardDraw(path, 0)
//     cardDraw.Replace(reader, false)
//     return cardDraw
// }

// func (s *ActionCardDraw) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 8: return &WireType.Varint
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Varint
//     case 5: return &WireType.Sized
//     case 6: return &WireType.Sized
//     case 7: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionCardDraw) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.Reshuffle
//     case 3: return &s.BeforeDraw
//     case 5: return &s.Draw
//     case 6: return &s.CardId
//     case 7: return &s.AfterDraw
//     default: return nil
//     }
// }

// func (s *ActionCardDraw) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.PlayerIndex = s.MaybeNotify(0, reader.Readuint32(), s.PlayerIndex, s.OnPlayerIndexUpdate, shouldNotify)
//     case 8: s.FullCardsInHand = s.MaybeNotify(8, reader.Readbool(), s.FullCardsInHand, s.OnFullCardsInHandUpdate, shouldNotify)
//     case 1: s.Reshuffle = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.Reshuffle, s.OnReshuffleUpdate, shouldNotify)
//     case 2: s.NoCardsLeft = s.MaybeNotify(2, reader.Readbool(), s.NoCardsLeft, s.OnNoCardsLeftUpdate, shouldNotify)
//     case 3: s.BeforeDraw = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.BeforeDraw, s.OnBeforeDrawUpdate, shouldNotify)
//     case 4: s.IsBlocked = s.MaybeNotify(4, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 5: s.Draw = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.Draw, s.OnDrawUpdate, shouldNotify)
//     case 6: s.CardId = s.MaybeNotify(6, OptionDeserialize(reader, s.Path.GetNested(6)), s.CardId, s.OnCardIdUpdate, shouldNotify)
//     case 7: s.AfterDraw = s.MaybeNotify(7, VectorDeserialize(reader, s.Path.GetNested(7)), s.AfterDraw, s.OnAfterDrawUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionCardDraw) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionCardDraw) ReplayListPop() { panic("") }
// func (s *ActionCardDraw) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionCardDraw) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (8): CardDiscard
// type ActionCardDiscard struct {
//     Path *Path
//     OnDiscarderIndexUpdate *EventHandler
//     OnOwnerIndexUpdate *EventHandler
//     OnCardIdUpdate *EventHandler
//     OnBeforeDiscardUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnDiscardUpdate *EventHandler
//     OnAfterDiscardUpdate *EventHandler
//     DiscarderIndex uint32
//     OwnerIndex uint32
//     CardId uint32
//     BeforeDiscard Vector
//     IsBlocked bool
//     Discard Vector
//     AfterDiscard Vector
// }

// func NewActionCardDiscard(path *Path, tag uint32) ActionCardDiscard {
//     obj := ActionCardDiscard{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.BeforeDiscard = NewVector(obj.Path.GetNested(3), 0)
//     obj.Discard = NewVector(obj.Path.GetNested(5), 0)
//     obj.AfterDiscard = NewVector(obj.Path.GetNested(6), 0)
//     return obj
// }

// func (s *ActionCardDiscard) ClearDiscarderIndexUpdateHandlers() { s.OnDiscarderIndexUpdate = nil }
// func (s *ActionCardDiscard) ClearOwnerIndexUpdateHandlers() { s.OnOwnerIndexUpdate = nil }
// func (s *ActionCardDiscard) ClearCardIdUpdateHandlers() { s.OnCardIdUpdate = nil }
// func (s *ActionCardDiscard) ClearBeforeDiscardUpdateHandlers() { s.OnBeforeDiscardUpdate = nil }
// func (s *ActionCardDiscard) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionCardDiscard) ClearDiscardUpdateHandlers() { s.OnDiscardUpdate = nil }
// func (s *ActionCardDiscard) ClearAfterDiscardUpdateHandlers() { s.OnAfterDiscardUpdate = nil }

// func (s *ActionCardDiscard) ClearUpdateHandlers() {
//     s.OnDiscarderIndexUpdate = nil
//     s.OnOwnerIndexUpdate = nil
//     s.OnCardIdUpdate = nil
//     s.OnBeforeDiscardUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnDiscardUpdate = nil
//     s.OnAfterDiscardUpdate = nil}

// func ActionCardDiscardDeserialize(_type reflect.Type, reader IReader, path *Path) ActionCardDiscard {
//     cardDiscard := NewActionCardDiscard(path, 0)
//     cardDiscard.Replace(reader, false)
//     return cardDiscard
// }

// func (s *ActionCardDiscard) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Varint
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Varint
//     case 5: return &WireType.Sized
//     case 6: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionCardDiscard) GetNested(tag uint32) *IState {
//     switch tag {
//     case 3: return &s.BeforeDiscard
//     case 5: return &s.Discard
//     case 6: return &s.AfterDiscard
//     default: return nil
//     }
// }

// func (s *ActionCardDiscard) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.DiscarderIndex = s.MaybeNotify(0, reader.Readuint32(), s.DiscarderIndex, s.OnDiscarderIndexUpdate, shouldNotify)
//     case 1: s.OwnerIndex = s.MaybeNotify(1, reader.Readuint32(), s.OwnerIndex, s.OnOwnerIndexUpdate, shouldNotify)
//     case 2: s.CardId = s.MaybeNotify(2, reader.Readuint32(), s.CardId, s.OnCardIdUpdate, shouldNotify)
//     case 3: s.BeforeDiscard = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.BeforeDiscard, s.OnBeforeDiscardUpdate, shouldNotify)
//     case 4: s.IsBlocked = s.MaybeNotify(4, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 5: s.Discard = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.Discard, s.OnDiscardUpdate, shouldNotify)
//     case 6: s.AfterDiscard = s.MaybeNotify(6, VectorDeserialize(reader, s.Path.GetNested(6)), s.AfterDiscard, s.OnAfterDiscardUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionCardDiscard) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionCardDiscard) ReplayListPop() { panic("") }
// func (s *ActionCardDiscard) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionCardDiscard) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (9): CardBanish
// type ActionCardBanish struct {
//     Path *Path
//     OnBanisherIndexUpdate *EventHandler
//     OnOwnerIndexUpdate *EventHandler
//     OnCardIdUpdate *EventHandler
//     OnBeforeBanishUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnBanishUpdate *EventHandler
//     OnAfterBanishUpdate *EventHandler
//     BanisherIndex uint32
//     OwnerIndex uint32
//     CardId uint32
//     BeforeBanish Vector
//     IsBlocked bool
//     Banish Vector
//     AfterBanish Vector
// }

// func NewActionCardBanish(path *Path, tag uint32) ActionCardBanish {
//     obj := ActionCardBanish{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.BeforeBanish = NewVector(obj.Path.GetNested(3), 0)
//     obj.Banish = NewVector(obj.Path.GetNested(5), 0)
//     obj.AfterBanish = NewVector(obj.Path.GetNested(6), 0)
//     return obj
// }

// func (s *ActionCardBanish) ClearBanisherIndexUpdateHandlers() { s.OnBanisherIndexUpdate = nil }
// func (s *ActionCardBanish) ClearOwnerIndexUpdateHandlers() { s.OnOwnerIndexUpdate = nil }
// func (s *ActionCardBanish) ClearCardIdUpdateHandlers() { s.OnCardIdUpdate = nil }
// func (s *ActionCardBanish) ClearBeforeBanishUpdateHandlers() { s.OnBeforeBanishUpdate = nil }
// func (s *ActionCardBanish) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionCardBanish) ClearBanishUpdateHandlers() { s.OnBanishUpdate = nil }
// func (s *ActionCardBanish) ClearAfterBanishUpdateHandlers() { s.OnAfterBanishUpdate = nil }

// func (s *ActionCardBanish) ClearUpdateHandlers() {
//     s.OnBanisherIndexUpdate = nil
//     s.OnOwnerIndexUpdate = nil
//     s.OnCardIdUpdate = nil
//     s.OnBeforeBanishUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnBanishUpdate = nil
//     s.OnAfterBanishUpdate = nil}

// func ActionCardBanishDeserialize(_type reflect.Type, reader IReader, path *Path) ActionCardBanish {
//     cardBanish := NewActionCardBanish(path, 0)
//     cardBanish.Replace(reader, false)
//     return cardBanish
// }

// func (s *ActionCardBanish) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Varint
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Varint
//     case 5: return &WireType.Sized
//     case 6: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionCardBanish) GetNested(tag uint32) *IState {
//     switch tag {
//     case 3: return &s.BeforeBanish
//     case 5: return &s.Banish
//     case 6: return &s.AfterBanish
//     default: return nil
//     }
// }

// func (s *ActionCardBanish) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.BanisherIndex = s.MaybeNotify(0, reader.Readuint32(), s.BanisherIndex, s.OnBanisherIndexUpdate, shouldNotify)
//     case 1: s.OwnerIndex = s.MaybeNotify(1, reader.Readuint32(), s.OwnerIndex, s.OnOwnerIndexUpdate, shouldNotify)
//     case 2: s.CardId = s.MaybeNotify(2, reader.Readuint32(), s.CardId, s.OnCardIdUpdate, shouldNotify)
//     case 3: s.BeforeBanish = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.BeforeBanish, s.OnBeforeBanishUpdate, shouldNotify)
//     case 4: s.IsBlocked = s.MaybeNotify(4, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 5: s.Banish = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.Banish, s.OnBanishUpdate, shouldNotify)
//     case 6: s.AfterBanish = s.MaybeNotify(6, VectorDeserialize(reader, s.Path.GetNested(6)), s.AfterBanish, s.OnAfterBanishUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionCardBanish) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionCardBanish) ReplayListPop() { panic("") }
// func (s *ActionCardBanish) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionCardBanish) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (11): CardPlay
// type ActionCardPlay struct {
//     Path *Path
//     OnPlayerIndexUpdate *EventHandler
//     OnCardIdUpdate *EventHandler
//     OnBeforeCardPlayUpdate *EventHandler
//     OnEnergyLossUpdate *EventHandler
//     OnCardMoveUpdate *EventHandler
//     OnAbilityUseUpdate *EventHandler
//     OnAfterCardPlayUpdate *EventHandler
//     PlayerIndex uint32
//     CardId uint32
//     BeforeCardPlay Vector
//     EnergyLoss Vector
//     CardMove Vector
//     AbilityUse Vector
//     AfterCardPlay Vector
// }

// func NewActionCardPlay(path *Path, tag uint32) ActionCardPlay {
//     obj := ActionCardPlay{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.BeforeCardPlay = NewVector(obj.Path.GetNested(2), 0)
//     obj.EnergyLoss = NewVector(obj.Path.GetNested(3), 0)
//     obj.CardMove = NewVector(obj.Path.GetNested(4), 0)
//     obj.AbilityUse = NewVector(obj.Path.GetNested(5), 0)
//     obj.AfterCardPlay = NewVector(obj.Path.GetNested(6), 0)
//     return obj
// }

// func (s *ActionCardPlay) ClearPlayerIndexUpdateHandlers() { s.OnPlayerIndexUpdate = nil }
// func (s *ActionCardPlay) ClearCardIdUpdateHandlers() { s.OnCardIdUpdate = nil }
// func (s *ActionCardPlay) ClearBeforeCardPlayUpdateHandlers() { s.OnBeforeCardPlayUpdate = nil }
// func (s *ActionCardPlay) ClearEnergyLossUpdateHandlers() { s.OnEnergyLossUpdate = nil }
// func (s *ActionCardPlay) ClearCardMoveUpdateHandlers() { s.OnCardMoveUpdate = nil }
// func (s *ActionCardPlay) ClearAbilityUseUpdateHandlers() { s.OnAbilityUseUpdate = nil }
// func (s *ActionCardPlay) ClearAfterCardPlayUpdateHandlers() { s.OnAfterCardPlayUpdate = nil }

// func (s *ActionCardPlay) ClearUpdateHandlers() {
//     s.OnPlayerIndexUpdate = nil
//     s.OnCardIdUpdate = nil
//     s.OnBeforeCardPlayUpdate = nil
//     s.OnEnergyLossUpdate = nil
//     s.OnCardMoveUpdate = nil
//     s.OnAbilityUseUpdate = nil
//     s.OnAfterCardPlayUpdate = nil}

// func ActionCardPlayDeserialize(_type reflect.Type, reader IReader, path *Path) ActionCardPlay {
//     cardPlay := NewActionCardPlay(path, 0)
//     cardPlay.Replace(reader, false)
//     return cardPlay
// }

// func (s *ActionCardPlay) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Varint
//     case 2: return &WireType.Sized
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Sized
//     case 5: return &WireType.Sized
//     case 6: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionCardPlay) GetNested(tag uint32) *IState {
//     switch tag {
//     case 2: return &s.BeforeCardPlay
//     case 3: return &s.EnergyLoss
//     case 4: return &s.CardMove
//     case 5: return &s.AbilityUse
//     case 6: return &s.AfterCardPlay
//     default: return nil
//     }
// }

// func (s *ActionCardPlay) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.PlayerIndex = s.MaybeNotify(0, reader.Readuint32(), s.PlayerIndex, s.OnPlayerIndexUpdate, shouldNotify)
//     case 1: s.CardId = s.MaybeNotify(1, reader.Readuint32(), s.CardId, s.OnCardIdUpdate, shouldNotify)
//     case 2: s.BeforeCardPlay = s.MaybeNotify(2, VectorDeserialize(reader, s.Path.GetNested(2)), s.BeforeCardPlay, s.OnBeforeCardPlayUpdate, shouldNotify)
//     case 3: s.EnergyLoss = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.EnergyLoss, s.OnEnergyLossUpdate, shouldNotify)
//     case 4: s.CardMove = s.MaybeNotify(4, VectorDeserialize(reader, s.Path.GetNested(4)), s.CardMove, s.OnCardMoveUpdate, shouldNotify)
//     case 5: s.AbilityUse = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.AbilityUse, s.OnAbilityUseUpdate, shouldNotify)
//     case 6: s.AfterCardPlay = s.MaybeNotify(6, VectorDeserialize(reader, s.Path.GetNested(6)), s.AfterCardPlay, s.OnAfterCardPlayUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionCardPlay) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionCardPlay) ReplayListPop() { panic("") }
// func (s *ActionCardPlay) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionCardPlay) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (12): StatusReveal
// type ActionStatusReveal struct {
//     Path *Path
//     OnPlayerIndexUpdate *EventHandler
//     OnCardIdUpdate *EventHandler
//     OnRevealUpdate *EventHandler
//     PlayerIndex uint32
//     CardId Option
//     Reveal Vector
// }

// func NewActionStatusReveal(path *Path, tag uint32) ActionStatusReveal {
//     obj := ActionStatusReveal{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.CardId = NewOption(obj.Path.GetNested(1), 0)
//     obj.Reveal = NewVector(obj.Path.GetNested(2), 0)
//     return obj
// }

// func (s *ActionStatusReveal) ClearPlayerIndexUpdateHandlers() { s.OnPlayerIndexUpdate = nil }
// func (s *ActionStatusReveal) ClearCardIdUpdateHandlers() { s.OnCardIdUpdate = nil }
// func (s *ActionStatusReveal) ClearRevealUpdateHandlers() { s.OnRevealUpdate = nil }

// func (s *ActionStatusReveal) ClearUpdateHandlers() {
//     s.OnPlayerIndexUpdate = nil
//     s.OnCardIdUpdate = nil
//     s.OnRevealUpdate = nil}

// func ActionStatusRevealDeserialize(_type reflect.Type, reader IReader, path *Path) ActionStatusReveal {
//     statusReveal := NewActionStatusReveal(path, 0)
//     statusReveal.Replace(reader, false)
//     return statusReveal
// }

// func (s *ActionStatusReveal) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionStatusReveal) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.CardId
//     case 2: return &s.Reveal
//     default: return nil
//     }
// }

// func (s *ActionStatusReveal) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.PlayerIndex = s.MaybeNotify(0, reader.Readuint32(), s.PlayerIndex, s.OnPlayerIndexUpdate, shouldNotify)
//     case 1: s.CardId = s.MaybeNotify(1, OptionDeserialize(reader, s.Path.GetNested(1)), s.CardId, s.OnCardIdUpdate, shouldNotify)
//     case 2: s.Reveal = s.MaybeNotify(2, VectorDeserialize(reader, s.Path.GetNested(2)), s.Reveal, s.OnRevealUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionStatusReveal) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionStatusReveal) ReplayListPop() { panic("") }
// func (s *ActionStatusReveal) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionStatusReveal) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (13): Attack
// type ActionAttack struct {
//     Path *Path
//     OnAttackerIndexUpdate *EventHandler
//     OnCardIdUpdate *EventHandler
//     OnBeforeAllAttacksUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnAttacksUpdate *EventHandler
//     OnAfterAllAttacksUpdate *EventHandler
//     OnChimeraAbilityUpdate *EventHandler
//     AttackerIndex uint32
//     CardId Option
//     BeforeAllAttacks Vector
//     IsBlocked bool
//     Attacks Vector
//     AfterAllAttacks Vector
//     ChimeraAbility Option
// }

// func NewActionAttack(path *Path, tag uint32) ActionAttack {
//     obj := ActionAttack{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.CardId = NewOption(obj.Path.GetNested(1), 0)
//     obj.BeforeAllAttacks = NewVector(obj.Path.GetNested(2), 0)
//     obj.Attacks = NewVector(obj.Path.GetNested(4), 0)
//     obj.AfterAllAttacks = NewVector(obj.Path.GetNested(5), 0)
//     obj.ChimeraAbility = NewOption(obj.Path.GetNested(6), 0)
//     return obj
// }

// func (s *ActionAttack) ClearAttackerIndexUpdateHandlers() { s.OnAttackerIndexUpdate = nil }
// func (s *ActionAttack) ClearCardIdUpdateHandlers() { s.OnCardIdUpdate = nil }
// func (s *ActionAttack) ClearBeforeAllAttacksUpdateHandlers() { s.OnBeforeAllAttacksUpdate = nil }
// func (s *ActionAttack) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionAttack) ClearAttacksUpdateHandlers() { s.OnAttacksUpdate = nil }
// func (s *ActionAttack) ClearAfterAllAttacksUpdateHandlers() { s.OnAfterAllAttacksUpdate = nil }
// func (s *ActionAttack) ClearChimeraAbilityUpdateHandlers() { s.OnChimeraAbilityUpdate = nil }

// func (s *ActionAttack) ClearUpdateHandlers() {
//     s.OnAttackerIndexUpdate = nil
//     s.OnCardIdUpdate = nil
//     s.OnBeforeAllAttacksUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnAttacksUpdate = nil
//     s.OnAfterAllAttacksUpdate = nil
//     s.OnChimeraAbilityUpdate = nil}

// func ActionAttackDeserialize(_type reflect.Type, reader IReader, path *Path) ActionAttack {
//     attack := NewActionAttack(path, 0)
//     attack.Replace(reader, false)
//     return attack
// }

// func (s *ActionAttack) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Sized
//     case 3: return &WireType.Varint
//     case 4: return &WireType.Sized
//     case 5: return &WireType.Sized
//     case 6: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionAttack) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.CardId
//     case 2: return &s.BeforeAllAttacks
//     case 4: return &s.Attacks
//     case 5: return &s.AfterAllAttacks
//     case 6: return &s.ChimeraAbility
//     default: return nil
//     }
// }

// func (s *ActionAttack) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.AttackerIndex = s.MaybeNotify(0, reader.Readuint32(), s.AttackerIndex, s.OnAttackerIndexUpdate, shouldNotify)
//     case 1: s.CardId = s.MaybeNotify(1, OptionDeserialize(reader, s.Path.GetNested(1)), s.CardId, s.OnCardIdUpdate, shouldNotify)
//     case 2: s.BeforeAllAttacks = s.MaybeNotify(2, VectorDeserialize(reader, s.Path.GetNested(2)), s.BeforeAllAttacks, s.OnBeforeAllAttacksUpdate, shouldNotify)
//     case 3: s.IsBlocked = s.MaybeNotify(3, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 4: s.Attacks = s.MaybeNotify(4, VectorDeserialize(reader, s.Path.GetNested(4)), s.Attacks, s.OnAttacksUpdate, shouldNotify)
//     case 5: s.AfterAllAttacks = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.AfterAllAttacks, s.OnAfterAllAttacksUpdate, shouldNotify)
//     case 6: s.ChimeraAbility = s.MaybeNotify(6, OptionDeserialize(reader, s.Path.GetNested(6)), s.ChimeraAbility, s.OnChimeraAbilityUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionAttack) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionAttack) ReplayListPop() { panic("") }
// func (s *ActionAttack) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionAttack) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (14): SkillCast
// type ActionSkillCast struct {
//     Path *Path
//     OnSkillCasterIndexUpdate *EventHandler
//     OnCardIdUpdate *EventHandler
//     OnBeforeAllSkillCastsUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnSkillCastsUpdate *EventHandler
//     OnAfterAllSkillCastsUpdate *EventHandler
//     OnChimeraAbilityUpdate *EventHandler
//     SkillCasterIndex Option
//     CardId Option
//     BeforeAllSkillCasts Vector
//     IsBlocked bool
//     SkillCasts Vector
//     AfterAllSkillCasts Vector
//     ChimeraAbility Option
// }

// func NewActionSkillCast(path *Path, tag uint32) ActionSkillCast {
//     obj := ActionSkillCast{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.SkillCasterIndex = NewOption(obj.Path.GetNested(0), 0)
//     obj.CardId = NewOption(obj.Path.GetNested(1), 0)
//     obj.BeforeAllSkillCasts = NewVector(obj.Path.GetNested(2), 0)
//     obj.SkillCasts = NewVector(obj.Path.GetNested(4), 0)
//     obj.AfterAllSkillCasts = NewVector(obj.Path.GetNested(5), 0)
//     obj.ChimeraAbility = NewOption(obj.Path.GetNested(6), 0)
//     return obj
// }

// func (s *ActionSkillCast) ClearSkillCasterIndexUpdateHandlers() { s.OnSkillCasterIndexUpdate = nil }
// func (s *ActionSkillCast) ClearCardIdUpdateHandlers() { s.OnCardIdUpdate = nil }
// func (s *ActionSkillCast) ClearBeforeAllSkillCastsUpdateHandlers() { s.OnBeforeAllSkillCastsUpdate = nil }
// func (s *ActionSkillCast) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionSkillCast) ClearSkillCastsUpdateHandlers() { s.OnSkillCastsUpdate = nil }
// func (s *ActionSkillCast) ClearAfterAllSkillCastsUpdateHandlers() { s.OnAfterAllSkillCastsUpdate = nil }
// func (s *ActionSkillCast) ClearChimeraAbilityUpdateHandlers() { s.OnChimeraAbilityUpdate = nil }

// func (s *ActionSkillCast) ClearUpdateHandlers() {
//     s.OnSkillCasterIndexUpdate = nil
//     s.OnCardIdUpdate = nil
//     s.OnBeforeAllSkillCastsUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnSkillCastsUpdate = nil
//     s.OnAfterAllSkillCastsUpdate = nil
//     s.OnChimeraAbilityUpdate = nil}

// func ActionSkillCastDeserialize(_type reflect.Type, reader IReader, path *Path) ActionSkillCast {
//     skillCast := NewActionSkillCast(path, 0)
//     skillCast.Replace(reader, false)
//     return skillCast
// }

// func (s *ActionSkillCast) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Sized
//     case 3: return &WireType.Varint
//     case 4: return &WireType.Sized
//     case 5: return &WireType.Sized
//     case 6: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionSkillCast) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.SkillCasterIndex
//     case 1: return &s.CardId
//     case 2: return &s.BeforeAllSkillCasts
//     case 4: return &s.SkillCasts
//     case 5: return &s.AfterAllSkillCasts
//     case 6: return &s.ChimeraAbility
//     default: return nil
//     }
// }

// func (s *ActionSkillCast) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.SkillCasterIndex = s.MaybeNotify(0, OptionDeserialize(reader, s.Path.GetNested(0)), s.SkillCasterIndex, s.OnSkillCasterIndexUpdate, shouldNotify)
//     case 1: s.CardId = s.MaybeNotify(1, OptionDeserialize(reader, s.Path.GetNested(1)), s.CardId, s.OnCardIdUpdate, shouldNotify)
//     case 2: s.BeforeAllSkillCasts = s.MaybeNotify(2, VectorDeserialize(reader, s.Path.GetNested(2)), s.BeforeAllSkillCasts, s.OnBeforeAllSkillCastsUpdate, shouldNotify)
//     case 3: s.IsBlocked = s.MaybeNotify(3, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 4: s.SkillCasts = s.MaybeNotify(4, VectorDeserialize(reader, s.Path.GetNested(4)), s.SkillCasts, s.OnSkillCastsUpdate, shouldNotify)
//     case 5: s.AfterAllSkillCasts = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.AfterAllSkillCasts, s.OnAfterAllSkillCastsUpdate, shouldNotify)
//     case 6: s.ChimeraAbility = s.MaybeNotify(6, OptionDeserialize(reader, s.Path.GetNested(6)), s.ChimeraAbility, s.OnChimeraAbilityUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionSkillCast) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionSkillCast) ReplayListPop() { panic("") }
// func (s *ActionSkillCast) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionSkillCast) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (15): Healing
// type ActionHealing struct {
//     Path *Path
//     OnSourceFighterIndexUpdate *EventHandler
//     OnSourceCardIdUpdate *EventHandler
//     OnSourceStatusIdUpdate *EventHandler
//     OnSourceChimeraAbilityUpdate *EventHandler
//     OnTargetFighterIndexUpdate *EventHandler
//     OnBeforeHealingUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnHealingUpdate *EventHandler
//     OnHealingAmountUpdate *EventHandler
//     OnAfterHealingUpdate *EventHandler
//     SourceFighterIndex Option
//     SourceCardId Option
//     SourceStatusId Option
//     SourceChimeraAbility Option
//     TargetFighterIndex uint32
//     BeforeHealing Vector
//     IsBlocked bool
//     Healing Vector
//     HealingAmount uint32
//     AfterHealing Vector
// }

// func NewActionHealing(path *Path, tag uint32) ActionHealing {
//     obj := ActionHealing{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.SourceFighterIndex = NewOption(obj.Path.GetNested(0), 0)
//     obj.SourceCardId = NewOption(obj.Path.GetNested(1), 0)
//     obj.SourceStatusId = NewOption(obj.Path.GetNested(2), 0)
//     obj.SourceChimeraAbility = NewOption(obj.Path.GetNested(9), 0)
//     obj.BeforeHealing = NewVector(obj.Path.GetNested(4), 0)
//     obj.Healing = NewVector(obj.Path.GetNested(6), 0)
//     obj.AfterHealing = NewVector(obj.Path.GetNested(8), 0)
//     return obj
// }

// func (s *ActionHealing) ClearSourceFighterIndexUpdateHandlers() { s.OnSourceFighterIndexUpdate = nil }
// func (s *ActionHealing) ClearSourceCardIdUpdateHandlers() { s.OnSourceCardIdUpdate = nil }
// func (s *ActionHealing) ClearSourceStatusIdUpdateHandlers() { s.OnSourceStatusIdUpdate = nil }
// func (s *ActionHealing) ClearSourceChimeraAbilityUpdateHandlers() { s.OnSourceChimeraAbilityUpdate = nil }
// func (s *ActionHealing) ClearTargetFighterIndexUpdateHandlers() { s.OnTargetFighterIndexUpdate = nil }
// func (s *ActionHealing) ClearBeforeHealingUpdateHandlers() { s.OnBeforeHealingUpdate = nil }
// func (s *ActionHealing) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionHealing) ClearHealingUpdateHandlers() { s.OnHealingUpdate = nil }
// func (s *ActionHealing) ClearHealingAmountUpdateHandlers() { s.OnHealingAmountUpdate = nil }
// func (s *ActionHealing) ClearAfterHealingUpdateHandlers() { s.OnAfterHealingUpdate = nil }

// func (s *ActionHealing) ClearUpdateHandlers() {
//     s.OnSourceFighterIndexUpdate = nil
//     s.OnSourceCardIdUpdate = nil
//     s.OnSourceStatusIdUpdate = nil
//     s.OnSourceChimeraAbilityUpdate = nil
//     s.OnTargetFighterIndexUpdate = nil
//     s.OnBeforeHealingUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnHealingUpdate = nil
//     s.OnHealingAmountUpdate = nil
//     s.OnAfterHealingUpdate = nil}

// func ActionHealingDeserialize(_type reflect.Type, reader IReader, path *Path) ActionHealing {
//     healing := NewActionHealing(path, 0)
//     healing.Replace(reader, false)
//     return healing
// }

// func (s *ActionHealing) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Sized
//     case 9: return &WireType.Sized
//     case 3: return &WireType.Varint
//     case 4: return &WireType.Sized
//     case 5: return &WireType.Varint
//     case 6: return &WireType.Sized
//     case 7: return &WireType.Varint
//     case 8: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionHealing) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.SourceFighterIndex
//     case 1: return &s.SourceCardId
//     case 2: return &s.SourceStatusId
//     case 9: return &s.SourceChimeraAbility
//     case 4: return &s.BeforeHealing
//     case 6: return &s.Healing
//     case 8: return &s.AfterHealing
//     default: return nil
//     }
// }

// func (s *ActionHealing) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.SourceFighterIndex = s.MaybeNotify(0, OptionDeserialize(reader, s.Path.GetNested(0)), s.SourceFighterIndex, s.OnSourceFighterIndexUpdate, shouldNotify)
//     case 1: s.SourceCardId = s.MaybeNotify(1, OptionDeserialize(reader, s.Path.GetNested(1)), s.SourceCardId, s.OnSourceCardIdUpdate, shouldNotify)
//     case 2: s.SourceStatusId = s.MaybeNotify(2, OptionDeserialize(reader, s.Path.GetNested(2)), s.SourceStatusId, s.OnSourceStatusIdUpdate, shouldNotify)
//     case 9: s.SourceChimeraAbility = s.MaybeNotify(9, OptionDeserialize(reader, s.Path.GetNested(9)), s.SourceChimeraAbility, s.OnSourceChimeraAbilityUpdate, shouldNotify)
//     case 3: s.TargetFighterIndex = s.MaybeNotify(3, reader.Readuint32(), s.TargetFighterIndex, s.OnTargetFighterIndexUpdate, shouldNotify)
//     case 4: s.BeforeHealing = s.MaybeNotify(4, VectorDeserialize(reader, s.Path.GetNested(4)), s.BeforeHealing, s.OnBeforeHealingUpdate, shouldNotify)
//     case 5: s.IsBlocked = s.MaybeNotify(5, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 6: s.Healing = s.MaybeNotify(6, VectorDeserialize(reader, s.Path.GetNested(6)), s.Healing, s.OnHealingUpdate, shouldNotify)
//     case 7: s.HealingAmount = s.MaybeNotify(7, reader.Readuint32(), s.HealingAmount, s.OnHealingAmountUpdate, shouldNotify)
//     case 8: s.AfterHealing = s.MaybeNotify(8, VectorDeserialize(reader, s.Path.GetNested(8)), s.AfterHealing, s.OnAfterHealingUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionHealing) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionHealing) ReplayListPop() { panic("") }
// func (s *ActionHealing) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionHealing) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (16): ShieldGain
// type ActionShieldGain struct {
//     Path *Path
//     OnSourceFighterIndexUpdate *EventHandler
//     OnSourceCardIdUpdate *EventHandler
//     OnSourceStatusIdUpdate *EventHandler
//     OnSourceChimeraAbilityUpdate *EventHandler
//     OnTargetFighterIndexUpdate *EventHandler
//     OnBeforeGainUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnGainUpdate *EventHandler
//     OnShieldAmountUpdate *EventHandler
//     OnAfterGainUpdate *EventHandler
//     SourceFighterIndex Option
//     SourceCardId Option
//     SourceStatusId Option
//     SourceChimeraAbility Option
//     TargetFighterIndex uint32
//     BeforeGain Vector
//     IsBlocked bool
//     Gain Vector
//     ShieldAmount uint32
//     AfterGain Vector
// }

// func NewActionShieldGain(path *Path, tag uint32) ActionShieldGain {
//     obj := ActionShieldGain{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.SourceFighterIndex = NewOption(obj.Path.GetNested(6), 0)
//     obj.SourceCardId = NewOption(obj.Path.GetNested(7), 0)
//     obj.SourceStatusId = NewOption(obj.Path.GetNested(8), 0)
//     obj.SourceChimeraAbility = NewOption(obj.Path.GetNested(9), 0)
//     obj.BeforeGain = NewVector(obj.Path.GetNested(1), 0)
//     obj.Gain = NewVector(obj.Path.GetNested(3), 0)
//     obj.AfterGain = NewVector(obj.Path.GetNested(5), 0)
//     return obj
// }

// func (s *ActionShieldGain) ClearSourceFighterIndexUpdateHandlers() { s.OnSourceFighterIndexUpdate = nil }
// func (s *ActionShieldGain) ClearSourceCardIdUpdateHandlers() { s.OnSourceCardIdUpdate = nil }
// func (s *ActionShieldGain) ClearSourceStatusIdUpdateHandlers() { s.OnSourceStatusIdUpdate = nil }
// func (s *ActionShieldGain) ClearSourceChimeraAbilityUpdateHandlers() { s.OnSourceChimeraAbilityUpdate = nil }
// func (s *ActionShieldGain) ClearTargetFighterIndexUpdateHandlers() { s.OnTargetFighterIndexUpdate = nil }
// func (s *ActionShieldGain) ClearBeforeGainUpdateHandlers() { s.OnBeforeGainUpdate = nil }
// func (s *ActionShieldGain) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionShieldGain) ClearGainUpdateHandlers() { s.OnGainUpdate = nil }
// func (s *ActionShieldGain) ClearShieldAmountUpdateHandlers() { s.OnShieldAmountUpdate = nil }
// func (s *ActionShieldGain) ClearAfterGainUpdateHandlers() { s.OnAfterGainUpdate = nil }

// func (s *ActionShieldGain) ClearUpdateHandlers() {
//     s.OnSourceFighterIndexUpdate = nil
//     s.OnSourceCardIdUpdate = nil
//     s.OnSourceStatusIdUpdate = nil
//     s.OnSourceChimeraAbilityUpdate = nil
//     s.OnTargetFighterIndexUpdate = nil
//     s.OnBeforeGainUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnGainUpdate = nil
//     s.OnShieldAmountUpdate = nil
//     s.OnAfterGainUpdate = nil}

// func ActionShieldGainDeserialize(_type reflect.Type, reader IReader, path *Path) ActionShieldGain {
//     shieldGain := NewActionShieldGain(path, 0)
//     shieldGain.Replace(reader, false)
//     return shieldGain
// }

// func (s *ActionShieldGain) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 6: return &WireType.Sized
//     case 7: return &WireType.Sized
//     case 8: return &WireType.Sized
//     case 9: return &WireType.Sized
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Varint
//     case 5: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionShieldGain) GetNested(tag uint32) *IState {
//     switch tag {
//     case 6: return &s.SourceFighterIndex
//     case 7: return &s.SourceCardId
//     case 8: return &s.SourceStatusId
//     case 9: return &s.SourceChimeraAbility
//     case 1: return &s.BeforeGain
//     case 3: return &s.Gain
//     case 5: return &s.AfterGain
//     default: return nil
//     }
// }

// func (s *ActionShieldGain) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 6: s.SourceFighterIndex = s.MaybeNotify(6, OptionDeserialize(reader, s.Path.GetNested(6)), s.SourceFighterIndex, s.OnSourceFighterIndexUpdate, shouldNotify)
//     case 7: s.SourceCardId = s.MaybeNotify(7, OptionDeserialize(reader, s.Path.GetNested(7)), s.SourceCardId, s.OnSourceCardIdUpdate, shouldNotify)
//     case 8: s.SourceStatusId = s.MaybeNotify(8, OptionDeserialize(reader, s.Path.GetNested(8)), s.SourceStatusId, s.OnSourceStatusIdUpdate, shouldNotify)
//     case 9: s.SourceChimeraAbility = s.MaybeNotify(9, OptionDeserialize(reader, s.Path.GetNested(9)), s.SourceChimeraAbility, s.OnSourceChimeraAbilityUpdate, shouldNotify)
//     case 0: s.TargetFighterIndex = s.MaybeNotify(0, reader.Readuint32(), s.TargetFighterIndex, s.OnTargetFighterIndexUpdate, shouldNotify)
//     case 1: s.BeforeGain = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.BeforeGain, s.OnBeforeGainUpdate, shouldNotify)
//     case 2: s.IsBlocked = s.MaybeNotify(2, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 3: s.Gain = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.Gain, s.OnGainUpdate, shouldNotify)
//     case 4: s.ShieldAmount = s.MaybeNotify(4, reader.Readuint32(), s.ShieldAmount, s.OnShieldAmountUpdate, shouldNotify)
//     case 5: s.AfterGain = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.AfterGain, s.OnAfterGainUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionShieldGain) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionShieldGain) ReplayListPop() { panic("") }
// func (s *ActionShieldGain) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionShieldGain) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (17): ShieldReduction
// type ActionShieldReduction struct {
//     Path *Path
//     OnFighterIndexUpdate *EventHandler
//     OnBeforeReductionUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnReductionUpdate *EventHandler
//     OnShieldAmountUpdate *EventHandler
//     OnAfterReductionUpdate *EventHandler
//     FighterIndex uint32
//     BeforeReduction Vector
//     IsBlocked bool
//     Reduction Vector
//     ShieldAmount uint32
//     AfterReduction Vector
// }

// func NewActionShieldReduction(path *Path, tag uint32) ActionShieldReduction {
//     obj := ActionShieldReduction{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.BeforeReduction = NewVector(obj.Path.GetNested(1), 0)
//     obj.Reduction = NewVector(obj.Path.GetNested(3), 0)
//     obj.AfterReduction = NewVector(obj.Path.GetNested(5), 0)
//     return obj
// }

// func (s *ActionShieldReduction) ClearFighterIndexUpdateHandlers() { s.OnFighterIndexUpdate = nil }
// func (s *ActionShieldReduction) ClearBeforeReductionUpdateHandlers() { s.OnBeforeReductionUpdate = nil }
// func (s *ActionShieldReduction) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionShieldReduction) ClearReductionUpdateHandlers() { s.OnReductionUpdate = nil }
// func (s *ActionShieldReduction) ClearShieldAmountUpdateHandlers() { s.OnShieldAmountUpdate = nil }
// func (s *ActionShieldReduction) ClearAfterReductionUpdateHandlers() { s.OnAfterReductionUpdate = nil }

// func (s *ActionShieldReduction) ClearUpdateHandlers() {
//     s.OnFighterIndexUpdate = nil
//     s.OnBeforeReductionUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnReductionUpdate = nil
//     s.OnShieldAmountUpdate = nil
//     s.OnAfterReductionUpdate = nil}

// func ActionShieldReductionDeserialize(_type reflect.Type, reader IReader, path *Path) ActionShieldReduction {
//     shieldReduction := NewActionShieldReduction(path, 0)
//     shieldReduction.Replace(reader, false)
//     return shieldReduction
// }

// func (s *ActionShieldReduction) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Varint
//     case 5: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionShieldReduction) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.BeforeReduction
//     case 3: return &s.Reduction
//     case 5: return &s.AfterReduction
//     default: return nil
//     }
// }

// func (s *ActionShieldReduction) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.FighterIndex = s.MaybeNotify(0, reader.Readuint32(), s.FighterIndex, s.OnFighterIndexUpdate, shouldNotify)
//     case 1: s.BeforeReduction = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.BeforeReduction, s.OnBeforeReductionUpdate, shouldNotify)
//     case 2: s.IsBlocked = s.MaybeNotify(2, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 3: s.Reduction = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.Reduction, s.OnReductionUpdate, shouldNotify)
//     case 4: s.ShieldAmount = s.MaybeNotify(4, reader.Readuint32(), s.ShieldAmount, s.OnShieldAmountUpdate, shouldNotify)
//     case 5: s.AfterReduction = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.AfterReduction, s.OnAfterReductionUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionShieldReduction) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionShieldReduction) ReplayListPop() { panic("") }
// func (s *ActionShieldReduction) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionShieldReduction) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (18): Summoning
// type ActionSummoning struct {
//     Path *Path
//     OnPlayerIndexUpdate *EventHandler
//     OnBeforeSummoningUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnSummoningUpdate *EventHandler
//     OnFighterIndexUpdate *EventHandler
//     OnAfterSummoningUpdate *EventHandler
//     PlayerIndex uint32
//     BeforeSummoning Vector
//     IsBlocked bool
//     Summoning Vector
//     FighterIndex Option
//     AfterSummoning Vector
// }

// func NewActionSummoning(path *Path, tag uint32) ActionSummoning {
//     obj := ActionSummoning{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.BeforeSummoning = NewVector(obj.Path.GetNested(1), 0)
//     obj.Summoning = NewVector(obj.Path.GetNested(3), 0)
//     obj.FighterIndex = NewOption(obj.Path.GetNested(4), 0)
//     obj.AfterSummoning = NewVector(obj.Path.GetNested(6), 0)
//     return obj
// }

// func (s *ActionSummoning) ClearPlayerIndexUpdateHandlers() { s.OnPlayerIndexUpdate = nil }
// func (s *ActionSummoning) ClearBeforeSummoningUpdateHandlers() { s.OnBeforeSummoningUpdate = nil }
// func (s *ActionSummoning) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionSummoning) ClearSummoningUpdateHandlers() { s.OnSummoningUpdate = nil }
// func (s *ActionSummoning) ClearFighterIndexUpdateHandlers() { s.OnFighterIndexUpdate = nil }
// func (s *ActionSummoning) ClearAfterSummoningUpdateHandlers() { s.OnAfterSummoningUpdate = nil }

// func (s *ActionSummoning) ClearUpdateHandlers() {
//     s.OnPlayerIndexUpdate = nil
//     s.OnBeforeSummoningUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnSummoningUpdate = nil
//     s.OnFighterIndexUpdate = nil
//     s.OnAfterSummoningUpdate = nil}

// func ActionSummoningDeserialize(_type reflect.Type, reader IReader, path *Path) ActionSummoning {
//     summoning := NewActionSummoning(path, 0)
//     summoning.Replace(reader, false)
//     return summoning
// }

// func (s *ActionSummoning) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Sized
//     case 6: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionSummoning) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.BeforeSummoning
//     case 3: return &s.Summoning
//     case 4: return &s.FighterIndex
//     case 6: return &s.AfterSummoning
//     default: return nil
//     }
// }

// func (s *ActionSummoning) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.PlayerIndex = s.MaybeNotify(0, reader.Readuint32(), s.PlayerIndex, s.OnPlayerIndexUpdate, shouldNotify)
//     case 1: s.BeforeSummoning = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.BeforeSummoning, s.OnBeforeSummoningUpdate, shouldNotify)
//     case 2: s.IsBlocked = s.MaybeNotify(2, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 3: s.Summoning = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.Summoning, s.OnSummoningUpdate, shouldNotify)
//     case 4: s.FighterIndex = s.MaybeNotify(4, OptionDeserialize(reader, s.Path.GetNested(4)), s.FighterIndex, s.OnFighterIndexUpdate, shouldNotify)
//     case 6: s.AfterSummoning = s.MaybeNotify(6, VectorDeserialize(reader, s.Path.GetNested(6)), s.AfterSummoning, s.OnAfterSummoningUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionSummoning) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionSummoning) ReplayListPop() { panic("") }
// func (s *ActionSummoning) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionSummoning) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (19): CardAddition
// type ActionCardAddition struct {
//     Path *Path
//     OnPlayerIndexUpdate *EventHandler
//     OnFullCardsInHandUpdate *EventHandler
//     OnBeforeCardAdditionUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnCardAdditionUpdate *EventHandler
//     OnCardIdUpdate *EventHandler
//     OnPileUpdate *EventHandler
//     OnAfterCardAdditionUpdate *EventHandler
//     PlayerIndex uint32
//     FullCardsInHand bool
//     BeforeCardAddition Vector
//     IsBlocked bool
//     CardAddition Vector
//     CardId Option
//     Pile Option
//     AfterCardAddition Vector
// }

// func NewActionCardAddition(path *Path, tag uint32) ActionCardAddition {
//     obj := ActionCardAddition{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.BeforeCardAddition = NewVector(obj.Path.GetNested(2), 0)
//     obj.CardAddition = NewVector(obj.Path.GetNested(4), 0)
//     obj.CardId = NewOption(obj.Path.GetNested(5), 0)
//     obj.Pile = NewOption(obj.Path.GetNested(6), 0)
//     obj.AfterCardAddition = NewVector(obj.Path.GetNested(7), 0)
//     return obj
// }

// func (s *ActionCardAddition) ClearPlayerIndexUpdateHandlers() { s.OnPlayerIndexUpdate = nil }
// func (s *ActionCardAddition) ClearFullCardsInHandUpdateHandlers() { s.OnFullCardsInHandUpdate = nil }
// func (s *ActionCardAddition) ClearBeforeCardAdditionUpdateHandlers() { s.OnBeforeCardAdditionUpdate = nil }
// func (s *ActionCardAddition) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionCardAddition) ClearCardAdditionUpdateHandlers() { s.OnCardAdditionUpdate = nil }
// func (s *ActionCardAddition) ClearCardIdUpdateHandlers() { s.OnCardIdUpdate = nil }
// func (s *ActionCardAddition) ClearPileUpdateHandlers() { s.OnPileUpdate = nil }
// func (s *ActionCardAddition) ClearAfterCardAdditionUpdateHandlers() { s.OnAfterCardAdditionUpdate = nil }

// func (s *ActionCardAddition) ClearUpdateHandlers() {
//     s.OnPlayerIndexUpdate = nil
//     s.OnFullCardsInHandUpdate = nil
//     s.OnBeforeCardAdditionUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnCardAdditionUpdate = nil
//     s.OnCardIdUpdate = nil
//     s.OnPileUpdate = nil
//     s.OnAfterCardAdditionUpdate = nil}

// func ActionCardAdditionDeserialize(_type reflect.Type, reader IReader, path *Path) ActionCardAddition {
//     cardAddition := NewActionCardAddition(path, 0)
//     cardAddition.Replace(reader, false)
//     return cardAddition
// }

// func (s *ActionCardAddition) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Varint
//     case 2: return &WireType.Sized
//     case 3: return &WireType.Varint
//     case 4: return &WireType.Sized
//     case 5: return &WireType.Sized
//     case 6: return &WireType.Sized
//     case 7: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionCardAddition) GetNested(tag uint32) *IState {
//     switch tag {
//     case 2: return &s.BeforeCardAddition
//     case 4: return &s.CardAddition
//     case 5: return &s.CardId
//     case 6: return &s.Pile
//     case 7: return &s.AfterCardAddition
//     default: return nil
//     }
// }

// func (s *ActionCardAddition) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.PlayerIndex = s.MaybeNotify(0, reader.Readuint32(), s.PlayerIndex, s.OnPlayerIndexUpdate, shouldNotify)
//     case 1: s.FullCardsInHand = s.MaybeNotify(1, reader.Readbool(), s.FullCardsInHand, s.OnFullCardsInHandUpdate, shouldNotify)
//     case 2: s.BeforeCardAddition = s.MaybeNotify(2, VectorDeserialize(reader, s.Path.GetNested(2)), s.BeforeCardAddition, s.OnBeforeCardAdditionUpdate, shouldNotify)
//     case 3: s.IsBlocked = s.MaybeNotify(3, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 4: s.CardAddition = s.MaybeNotify(4, VectorDeserialize(reader, s.Path.GetNested(4)), s.CardAddition, s.OnCardAdditionUpdate, shouldNotify)
//     case 5: s.CardId = s.MaybeNotify(5, OptionDeserialize(reader, s.Path.GetNested(5)), s.CardId, s.OnCardIdUpdate, shouldNotify)
//     case 6: s.Pile = s.MaybeNotify(6, OptionDeserialize(reader, s.Path.GetNested(6)), s.Pile, s.OnPileUpdate, shouldNotify)
//     case 7: s.AfterCardAddition = s.MaybeNotify(7, VectorDeserialize(reader, s.Path.GetNested(7)), s.AfterCardAddition, s.OnAfterCardAdditionUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionCardAddition) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionCardAddition) ReplayListPop() { panic("") }
// func (s *ActionCardAddition) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionCardAddition) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (20): CardRevelation
// type ActionCardRevelation struct {
//     Path *Path
//     OnBeforeCardRevelationUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnCardsUpdate *EventHandler
//     OnPileUpdate *EventHandler
//     OnAfterCardRevelationUpdate *EventHandler
//     BeforeCardRevelation Vector
//     IsBlocked bool
//     Cards Vector
//     Pile Option
//     AfterCardRevelation Vector
// }

// func NewActionCardRevelation(path *Path, tag uint32) ActionCardRevelation {
//     obj := ActionCardRevelation{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.BeforeCardRevelation = NewVector(obj.Path.GetNested(0), 0)
//     obj.Cards = NewVector(obj.Path.GetNested(2), 0)
//     obj.Pile = NewOption(obj.Path.GetNested(3), 0)
//     obj.AfterCardRevelation = NewVector(obj.Path.GetNested(4), 0)
//     return obj
// }

// func (s *ActionCardRevelation) ClearBeforeCardRevelationUpdateHandlers() { s.OnBeforeCardRevelationUpdate = nil }
// func (s *ActionCardRevelation) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionCardRevelation) ClearCardsUpdateHandlers() { s.OnCardsUpdate = nil }
// func (s *ActionCardRevelation) ClearPileUpdateHandlers() { s.OnPileUpdate = nil }
// func (s *ActionCardRevelation) ClearAfterCardRevelationUpdateHandlers() { s.OnAfterCardRevelationUpdate = nil }

// func (s *ActionCardRevelation) ClearUpdateHandlers() {
//     s.OnBeforeCardRevelationUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnCardsUpdate = nil
//     s.OnPileUpdate = nil
//     s.OnAfterCardRevelationUpdate = nil}

// func ActionCardRevelationDeserialize(_type reflect.Type, reader IReader, path *Path) ActionCardRevelation {
//     cardRevelation := NewActionCardRevelation(path, 0)
//     cardRevelation.Replace(reader, false)
//     return cardRevelation
// }

// func (s *ActionCardRevelation) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     case 1: return &WireType.Varint
//     case 2: return &WireType.Sized
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionCardRevelation) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.BeforeCardRevelation
//     case 2: return &s.Cards
//     case 3: return &s.Pile
//     case 4: return &s.AfterCardRevelation
//     default: return nil
//     }
// }

// func (s *ActionCardRevelation) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.BeforeCardRevelation = s.MaybeNotify(0, VectorDeserialize(reader, s.Path.GetNested(0)), s.BeforeCardRevelation, s.OnBeforeCardRevelationUpdate, shouldNotify)
//     case 1: s.IsBlocked = s.MaybeNotify(1, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 2: s.Cards = s.MaybeNotify(2, VectorDeserialize(reader, s.Path.GetNested(2)), s.Cards, s.OnCardsUpdate, shouldNotify)
//     case 3: s.Pile = s.MaybeNotify(3, OptionDeserialize(reader, s.Path.GetNested(3)), s.Pile, s.OnPileUpdate, shouldNotify)
//     case 4: s.AfterCardRevelation = s.MaybeNotify(4, VectorDeserialize(reader, s.Path.GetNested(4)), s.AfterCardRevelation, s.OnAfterCardRevelationUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionCardRevelation) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionCardRevelation) ReplayListPop() { panic("") }
// func (s *ActionCardRevelation) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionCardRevelation) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (21): StatusCast
// type ActionStatusCast struct {
//     Path *Path
//     OnCardIdUpdate *EventHandler
//     OnBeforeStatusCastUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnStatusCastUpdate *EventHandler
//     OnAfterStatusCastUpdate *EventHandler
//     OnChimeraAbilityUpdate *EventHandler
//     CardId Option
//     BeforeStatusCast Vector
//     IsBlocked bool
//     StatusCast Vector
//     AfterStatusCast Vector
//     ChimeraAbility Option
// }

// func NewActionStatusCast(path *Path, tag uint32) ActionStatusCast {
//     obj := ActionStatusCast{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.CardId = NewOption(obj.Path.GetNested(0), 0)
//     obj.BeforeStatusCast = NewVector(obj.Path.GetNested(1), 0)
//     obj.StatusCast = NewVector(obj.Path.GetNested(3), 0)
//     obj.AfterStatusCast = NewVector(obj.Path.GetNested(4), 0)
//     obj.ChimeraAbility = NewOption(obj.Path.GetNested(5), 0)
//     return obj
// }

// func (s *ActionStatusCast) ClearCardIdUpdateHandlers() { s.OnCardIdUpdate = nil }
// func (s *ActionStatusCast) ClearBeforeStatusCastUpdateHandlers() { s.OnBeforeStatusCastUpdate = nil }
// func (s *ActionStatusCast) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionStatusCast) ClearStatusCastUpdateHandlers() { s.OnStatusCastUpdate = nil }
// func (s *ActionStatusCast) ClearAfterStatusCastUpdateHandlers() { s.OnAfterStatusCastUpdate = nil }
// func (s *ActionStatusCast) ClearChimeraAbilityUpdateHandlers() { s.OnChimeraAbilityUpdate = nil }

// func (s *ActionStatusCast) ClearUpdateHandlers() {
//     s.OnCardIdUpdate = nil
//     s.OnBeforeStatusCastUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnStatusCastUpdate = nil
//     s.OnAfterStatusCastUpdate = nil
//     s.OnChimeraAbilityUpdate = nil}

// func ActionStatusCastDeserialize(_type reflect.Type, reader IReader, path *Path) ActionStatusCast {
//     statusCast := NewActionStatusCast(path, 0)
//     statusCast.Replace(reader, false)
//     return statusCast
// }

// func (s *ActionStatusCast) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Sized
//     case 5: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionStatusCast) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.CardId
//     case 1: return &s.BeforeStatusCast
//     case 3: return &s.StatusCast
//     case 4: return &s.AfterStatusCast
//     case 5: return &s.ChimeraAbility
//     default: return nil
//     }
// }

// func (s *ActionStatusCast) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.CardId = s.MaybeNotify(0, OptionDeserialize(reader, s.Path.GetNested(0)), s.CardId, s.OnCardIdUpdate, shouldNotify)
//     case 1: s.BeforeStatusCast = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.BeforeStatusCast, s.OnBeforeStatusCastUpdate, shouldNotify)
//     case 2: s.IsBlocked = s.MaybeNotify(2, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 3: s.StatusCast = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.StatusCast, s.OnStatusCastUpdate, shouldNotify)
//     case 4: s.AfterStatusCast = s.MaybeNotify(4, VectorDeserialize(reader, s.Path.GetNested(4)), s.AfterStatusCast, s.OnAfterStatusCastUpdate, shouldNotify)
//     case 5: s.ChimeraAbility = s.MaybeNotify(5, OptionDeserialize(reader, s.Path.GetNested(5)), s.ChimeraAbility, s.OnChimeraAbilityUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionStatusCast) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionStatusCast) ReplayListPop() { panic("") }
// func (s *ActionStatusCast) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionStatusCast) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (22): PowerCast
// type ActionPowerCast struct {
//     Path *Path
//     OnPowerCasterIndexUpdate *EventHandler
//     OnCardIdUpdate *EventHandler
//     OnBeforeAllPowerCastsUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnPowerCastsUpdate *EventHandler
//     OnAfterAllPowerCastsUpdate *EventHandler
//     OnChimeraAbilityUpdate *EventHandler
//     PowerCasterIndex Option
//     CardId Option
//     BeforeAllPowerCasts Vector
//     IsBlocked bool
//     PowerCasts Vector
//     AfterAllPowerCasts Vector
//     ChimeraAbility Option
// }

// func NewActionPowerCast(path *Path, tag uint32) ActionPowerCast {
//     obj := ActionPowerCast{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.PowerCasterIndex = NewOption(obj.Path.GetNested(0), 0)
//     obj.CardId = NewOption(obj.Path.GetNested(1), 0)
//     obj.BeforeAllPowerCasts = NewVector(obj.Path.GetNested(2), 0)
//     obj.PowerCasts = NewVector(obj.Path.GetNested(4), 0)
//     obj.AfterAllPowerCasts = NewVector(obj.Path.GetNested(5), 0)
//     obj.ChimeraAbility = NewOption(obj.Path.GetNested(6), 0)
//     return obj
// }

// func (s *ActionPowerCast) ClearPowerCasterIndexUpdateHandlers() { s.OnPowerCasterIndexUpdate = nil }
// func (s *ActionPowerCast) ClearCardIdUpdateHandlers() { s.OnCardIdUpdate = nil }
// func (s *ActionPowerCast) ClearBeforeAllPowerCastsUpdateHandlers() { s.OnBeforeAllPowerCastsUpdate = nil }
// func (s *ActionPowerCast) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionPowerCast) ClearPowerCastsUpdateHandlers() { s.OnPowerCastsUpdate = nil }
// func (s *ActionPowerCast) ClearAfterAllPowerCastsUpdateHandlers() { s.OnAfterAllPowerCastsUpdate = nil }
// func (s *ActionPowerCast) ClearChimeraAbilityUpdateHandlers() { s.OnChimeraAbilityUpdate = nil }

// func (s *ActionPowerCast) ClearUpdateHandlers() {
//     s.OnPowerCasterIndexUpdate = nil
//     s.OnCardIdUpdate = nil
//     s.OnBeforeAllPowerCastsUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnPowerCastsUpdate = nil
//     s.OnAfterAllPowerCastsUpdate = nil
//     s.OnChimeraAbilityUpdate = nil}

// func ActionPowerCastDeserialize(_type reflect.Type, reader IReader, path *Path) ActionPowerCast {
//     powerCast := NewActionPowerCast(path, 0)
//     powerCast.Replace(reader, false)
//     return powerCast
// }

// func (s *ActionPowerCast) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Sized
//     case 3: return &WireType.Varint
//     case 4: return &WireType.Sized
//     case 5: return &WireType.Sized
//     case 6: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionPowerCast) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.PowerCasterIndex
//     case 1: return &s.CardId
//     case 2: return &s.BeforeAllPowerCasts
//     case 4: return &s.PowerCasts
//     case 5: return &s.AfterAllPowerCasts
//     case 6: return &s.ChimeraAbility
//     default: return nil
//     }
// }

// func (s *ActionPowerCast) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.PowerCasterIndex = s.MaybeNotify(0, OptionDeserialize(reader, s.Path.GetNested(0)), s.PowerCasterIndex, s.OnPowerCasterIndexUpdate, shouldNotify)
//     case 1: s.CardId = s.MaybeNotify(1, OptionDeserialize(reader, s.Path.GetNested(1)), s.CardId, s.OnCardIdUpdate, shouldNotify)
//     case 2: s.BeforeAllPowerCasts = s.MaybeNotify(2, VectorDeserialize(reader, s.Path.GetNested(2)), s.BeforeAllPowerCasts, s.OnBeforeAllPowerCastsUpdate, shouldNotify)
//     case 3: s.IsBlocked = s.MaybeNotify(3, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 4: s.PowerCasts = s.MaybeNotify(4, VectorDeserialize(reader, s.Path.GetNested(4)), s.PowerCasts, s.OnPowerCastsUpdate, shouldNotify)
//     case 5: s.AfterAllPowerCasts = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.AfterAllPowerCasts, s.OnAfterAllPowerCastsUpdate, shouldNotify)
//     case 6: s.ChimeraAbility = s.MaybeNotify(6, OptionDeserialize(reader, s.Path.GetNested(6)), s.ChimeraAbility, s.OnChimeraAbilityUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionPowerCast) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionPowerCast) ReplayListPop() { panic("") }
// func (s *ActionPowerCast) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionPowerCast) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (23): Scry
// type ActionScry struct {
//     Path *Path
//     OnPlayerIndexUpdate *EventHandler
//     OnBeforeScryUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnScryUpdate *EventHandler
//     OnAmountUpdate *EventHandler
//     OnAfterScryUpdate *EventHandler
//     PlayerIndex uint32
//     BeforeScry Vector
//     IsBlocked bool
//     Scry Vector
//     Amount Option
//     AfterScry Vector
// }

// func NewActionScry(path *Path, tag uint32) ActionScry {
//     obj := ActionScry{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.BeforeScry = NewVector(obj.Path.GetNested(1), 0)
//     obj.Scry = NewVector(obj.Path.GetNested(3), 0)
//     obj.Amount = NewOption(obj.Path.GetNested(4), 0)
//     obj.AfterScry = NewVector(obj.Path.GetNested(5), 0)
//     return obj
// }

// func (s *ActionScry) ClearPlayerIndexUpdateHandlers() { s.OnPlayerIndexUpdate = nil }
// func (s *ActionScry) ClearBeforeScryUpdateHandlers() { s.OnBeforeScryUpdate = nil }
// func (s *ActionScry) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionScry) ClearScryUpdateHandlers() { s.OnScryUpdate = nil }
// func (s *ActionScry) ClearAmountUpdateHandlers() { s.OnAmountUpdate = nil }
// func (s *ActionScry) ClearAfterScryUpdateHandlers() { s.OnAfterScryUpdate = nil }

// func (s *ActionScry) ClearUpdateHandlers() {
//     s.OnPlayerIndexUpdate = nil
//     s.OnBeforeScryUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnScryUpdate = nil
//     s.OnAmountUpdate = nil
//     s.OnAfterScryUpdate = nil}

// func ActionScryDeserialize(_type reflect.Type, reader IReader, path *Path) ActionScry {
//     scry := NewActionScry(path, 0)
//     scry.Replace(reader, false)
//     return scry
// }

// func (s *ActionScry) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Sized
//     case 5: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionScry) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.BeforeScry
//     case 3: return &s.Scry
//     case 4: return &s.Amount
//     case 5: return &s.AfterScry
//     default: return nil
//     }
// }

// func (s *ActionScry) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.PlayerIndex = s.MaybeNotify(0, reader.Readuint32(), s.PlayerIndex, s.OnPlayerIndexUpdate, shouldNotify)
//     case 1: s.BeforeScry = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.BeforeScry, s.OnBeforeScryUpdate, shouldNotify)
//     case 2: s.IsBlocked = s.MaybeNotify(2, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 3: s.Scry = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.Scry, s.OnScryUpdate, shouldNotify)
//     case 4: s.Amount = s.MaybeNotify(4, OptionDeserialize(reader, s.Path.GetNested(4)), s.Amount, s.OnAmountUpdate, shouldNotify)
//     case 5: s.AfterScry = s.MaybeNotify(5, VectorDeserialize(reader, s.Path.GetNested(5)), s.AfterScry, s.OnAfterScryUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionScry) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionScry) ReplayListPop() { panic("") }
// func (s *ActionScry) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionScry) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (24): CurseCast
// type ActionCurseCast struct {
//     Path *Path
//     OnCardIdUpdate *EventHandler
//     OnBeforeCurseCastUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnCurseCastUpdate *EventHandler
//     OnAfterCurseCastUpdate *EventHandler
//     OnChimeraAbilityUpdate *EventHandler
//     CardId Option
//     BeforeCurseCast Vector
//     IsBlocked bool
//     CurseCast Vector
//     AfterCurseCast Vector
//     ChimeraAbility Option
// }

// func NewActionCurseCast(path *Path, tag uint32) ActionCurseCast {
//     obj := ActionCurseCast{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.CardId = NewOption(obj.Path.GetNested(0), 0)
//     obj.BeforeCurseCast = NewVector(obj.Path.GetNested(1), 0)
//     obj.CurseCast = NewVector(obj.Path.GetNested(3), 0)
//     obj.AfterCurseCast = NewVector(obj.Path.GetNested(4), 0)
//     obj.ChimeraAbility = NewOption(obj.Path.GetNested(5), 0)
//     return obj
// }

// func (s *ActionCurseCast) ClearCardIdUpdateHandlers() { s.OnCardIdUpdate = nil }
// func (s *ActionCurseCast) ClearBeforeCurseCastUpdateHandlers() { s.OnBeforeCurseCastUpdate = nil }
// func (s *ActionCurseCast) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionCurseCast) ClearCurseCastUpdateHandlers() { s.OnCurseCastUpdate = nil }
// func (s *ActionCurseCast) ClearAfterCurseCastUpdateHandlers() { s.OnAfterCurseCastUpdate = nil }
// func (s *ActionCurseCast) ClearChimeraAbilityUpdateHandlers() { s.OnChimeraAbilityUpdate = nil }

// func (s *ActionCurseCast) ClearUpdateHandlers() {
//     s.OnCardIdUpdate = nil
//     s.OnBeforeCurseCastUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnCurseCastUpdate = nil
//     s.OnAfterCurseCastUpdate = nil
//     s.OnChimeraAbilityUpdate = nil}

// func ActionCurseCastDeserialize(_type reflect.Type, reader IReader, path *Path) ActionCurseCast {
//     curseCast := NewActionCurseCast(path, 0)
//     curseCast.Replace(reader, false)
//     return curseCast
// }

// func (s *ActionCurseCast) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Varint
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Sized
//     case 5: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionCurseCast) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.CardId
//     case 1: return &s.BeforeCurseCast
//     case 3: return &s.CurseCast
//     case 4: return &s.AfterCurseCast
//     case 5: return &s.ChimeraAbility
//     default: return nil
//     }
// }

// func (s *ActionCurseCast) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.CardId = s.MaybeNotify(0, OptionDeserialize(reader, s.Path.GetNested(0)), s.CardId, s.OnCardIdUpdate, shouldNotify)
//     case 1: s.BeforeCurseCast = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.BeforeCurseCast, s.OnBeforeCurseCastUpdate, shouldNotify)
//     case 2: s.IsBlocked = s.MaybeNotify(2, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 3: s.CurseCast = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.CurseCast, s.OnCurseCastUpdate, shouldNotify)
//     case 4: s.AfterCurseCast = s.MaybeNotify(4, VectorDeserialize(reader, s.Path.GetNested(4)), s.AfterCurseCast, s.OnAfterCurseCastUpdate, shouldNotify)
//     case 5: s.ChimeraAbility = s.MaybeNotify(5, OptionDeserialize(reader, s.Path.GetNested(5)), s.ChimeraAbility, s.OnChimeraAbilityUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionCurseCast) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionCurseCast) ReplayListPop() { panic("") }
// func (s *ActionCurseCast) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionCurseCast) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (25): StatusAddition
// type ActionStatusAddition struct {
//     Path *Path
//     OnBeforeStatusAdditionUpdate *EventHandler
//     OnIsBlockedUpdate *EventHandler
//     OnStatusAdditionUpdate *EventHandler
//     OnStatusIdUpdate *EventHandler
//     OnAfterStatusAdditionUpdate *EventHandler
//     BeforeStatusAddition Vector
//     IsBlocked bool
//     StatusAddition Vector
//     StatusId Option
//     AfterStatusAddition Vector
// }

// func NewActionStatusAddition(path *Path, tag uint32) ActionStatusAddition {
//     obj := ActionStatusAddition{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.BeforeStatusAddition = NewVector(obj.Path.GetNested(0), 0)
//     obj.StatusAddition = NewVector(obj.Path.GetNested(2), 0)
//     obj.StatusId = NewOption(obj.Path.GetNested(3), 0)
//     obj.AfterStatusAddition = NewVector(obj.Path.GetNested(4), 0)
//     return obj
// }

// func (s *ActionStatusAddition) ClearBeforeStatusAdditionUpdateHandlers() { s.OnBeforeStatusAdditionUpdate = nil }
// func (s *ActionStatusAddition) ClearIsBlockedUpdateHandlers() { s.OnIsBlockedUpdate = nil }
// func (s *ActionStatusAddition) ClearStatusAdditionUpdateHandlers() { s.OnStatusAdditionUpdate = nil }
// func (s *ActionStatusAddition) ClearStatusIdUpdateHandlers() { s.OnStatusIdUpdate = nil }
// func (s *ActionStatusAddition) ClearAfterStatusAdditionUpdateHandlers() { s.OnAfterStatusAdditionUpdate = nil }

// func (s *ActionStatusAddition) ClearUpdateHandlers() {
//     s.OnBeforeStatusAdditionUpdate = nil
//     s.OnIsBlockedUpdate = nil
//     s.OnStatusAdditionUpdate = nil
//     s.OnStatusIdUpdate = nil
//     s.OnAfterStatusAdditionUpdate = nil}

// func ActionStatusAdditionDeserialize(_type reflect.Type, reader IReader, path *Path) ActionStatusAddition {
//     statusAddition := NewActionStatusAddition(path, 0)
//     statusAddition.Replace(reader, false)
//     return statusAddition
// }

// func (s *ActionStatusAddition) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     case 1: return &WireType.Varint
//     case 2: return &WireType.Sized
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionStatusAddition) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.BeforeStatusAddition
//     case 2: return &s.StatusAddition
//     case 3: return &s.StatusId
//     case 4: return &s.AfterStatusAddition
//     default: return nil
//     }
// }

// func (s *ActionStatusAddition) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.BeforeStatusAddition = s.MaybeNotify(0, VectorDeserialize(reader, s.Path.GetNested(0)), s.BeforeStatusAddition, s.OnBeforeStatusAdditionUpdate, shouldNotify)
//     case 1: s.IsBlocked = s.MaybeNotify(1, reader.Readbool(), s.IsBlocked, s.OnIsBlockedUpdate, shouldNotify)
//     case 2: s.StatusAddition = s.MaybeNotify(2, VectorDeserialize(reader, s.Path.GetNested(2)), s.StatusAddition, s.OnStatusAdditionUpdate, shouldNotify)
//     case 3: s.StatusId = s.MaybeNotify(3, OptionDeserialize(reader, s.Path.GetNested(3)), s.StatusId, s.OnStatusIdUpdate, shouldNotify)
//     case 4: s.AfterStatusAddition = s.MaybeNotify(4, VectorDeserialize(reader, s.Path.GetNested(4)), s.AfterStatusAddition, s.OnAfterStatusAdditionUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionStatusAddition) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionStatusAddition) ReplayListPop() { panic("") }
// func (s *ActionStatusAddition) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionStatusAddition) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (26): BloodMoonCurse
// type ActionBloodMoonCurse struct {
//     Path *Path
//     OnCurseUpdate *EventHandler
//     Curse Vector
// }

// func NewActionBloodMoonCurse(path *Path, tag uint32) ActionBloodMoonCurse {
//     obj := ActionBloodMoonCurse{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.Curse = NewVector(obj.Path.GetNested(0), 0)
//     return obj
// }

// func (s *ActionBloodMoonCurse) ClearCurseUpdateHandlers() {
//     s.OnCurseUpdate = nil
// }

// func (s *ActionBloodMoonCurse) ClearUpdateHandlers() {
//     s.OnCurseUpdate = nil}

// func ActionBloodMoonCurseDeserialize(_type reflect.Type, reader IReader, path *Path) ActionBloodMoonCurse {
//     bloodMoonCurse := NewActionBloodMoonCurse(path, 0)
//     bloodMoonCurse.Replace(reader, false)
//     return bloodMoonCurse
// }

// func (s *ActionBloodMoonCurse) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionBloodMoonCurse) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.Curse
//     default: return nil
//     }
// }

// func (s *ActionBloodMoonCurse) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.Curse = s.MaybeNotify(0, VectorDeserialize(reader, s.Path.GetNested(0)), s.Curse, s.OnCurseUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionBloodMoonCurse) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionBloodMoonCurse) ReplayListPop() { panic("") }
// func (s *ActionBloodMoonCurse) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionBloodMoonCurse) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (27): ApplyPoison
// type ActionApplyPoison struct {
//     Path *Path
//     OnFighterIndexUpdate *EventHandler
//     OnStacksUpdate *EventHandler
//     OnApplyUpdate *EventHandler
//     FighterIndex uint32
//     Stacks uint32
//     Apply Vector
// }

// func NewActionApplyPoison(path *Path, tag uint32) ActionApplyPoison {
//     obj := ActionApplyPoison{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.Apply = NewVector(obj.Path.GetNested(2), 0)
//     return obj
// }

// func (s *ActionApplyPoison) ClearFighterIndexUpdateHandlers() { s.OnFighterIndexUpdate = nil }
// func (s *ActionApplyPoison) ClearStacksUpdateHandlers() { s.OnStacksUpdate = nil }
// func (s *ActionApplyPoison) ClearApplyUpdateHandlers() { s.OnApplyUpdate = nil }

// func (s *ActionApplyPoison) ClearUpdateHandlers() {
//     s.OnFighterIndexUpdate = nil
//     s.OnStacksUpdate = nil
//     s.OnApplyUpdate = nil}

// func ActionApplyPoisonDeserialize(_type reflect.Type, reader IReader, path *Path) ActionApplyPoison {
//     applyPoison := NewActionApplyPoison(path, 0)
//     applyPoison.Replace(reader, false)
//     return applyPoison
// }

// func (s *ActionApplyPoison) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Varint
//     case 2: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionApplyPoison) GetNested(tag uint32) *IState {
//     switch tag {
//     case 2: return &s.Apply
//     default: return nil
//     }
// }

// func (s *ActionApplyPoison) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.FighterIndex = s.MaybeNotify(0, reader.Readuint32(), s.FighterIndex, s.OnFighterIndexUpdate, shouldNotify)
//     case 1: s.Stacks = s.MaybeNotify(1, reader.Readuint32(), s.Stacks, s.OnStacksUpdate, shouldNotify)
//     case 2: s.Apply = s.MaybeNotify(2, VectorDeserialize(reader, s.Path.GetNested(2)), s.Apply, s.OnApplyUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionApplyPoison) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionApplyPoison) ReplayListPop() { panic("") }
// func (s *ActionApplyPoison) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionApplyPoison) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (28): ApplyRage
// type ActionApplyRage struct {
//     Path *Path
//     OnFighterIndexUpdate *EventHandler
//     OnStacksUpdate *EventHandler
//     OnApplyUpdate *EventHandler
//     FighterIndex uint32
//     Stacks uint32
//     Apply Vector
// }

// func NewActionApplyRage(path *Path, tag uint32) ActionApplyRage {
//     obj := ActionApplyRage{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.Apply = NewVector(obj.Path.GetNested(2), 0)
//     return obj
// }

// func (s *ActionApplyRage) ClearFighterIndexUpdateHandlers() { s.OnFighterIndexUpdate = nil }
// func (s *ActionApplyRage) ClearStacksUpdateHandlers() { s.OnStacksUpdate = nil }
// func (s *ActionApplyRage) ClearApplyUpdateHandlers() { s.OnApplyUpdate = nil }

// func (s *ActionApplyRage) ClearUpdateHandlers() {
//     s.OnFighterIndexUpdate = nil
//     s.OnStacksUpdate = nil
//     s.OnApplyUpdate = nil}

// func ActionApplyRageDeserialize(_type reflect.Type, reader IReader, path *Path) ActionApplyRage {
//     applyRage := NewActionApplyRage(path, 0)
//     applyRage.Replace(reader, false)
//     return applyRage
// }

// func (s *ActionApplyRage) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Varint
//     case 2: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionApplyRage) GetNested(tag uint32) *IState {
//     switch tag {
//     case 2: return &s.Apply
//     default: return nil
//     }
// }

// func (s *ActionApplyRage) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.FighterIndex = s.MaybeNotify(0, reader.Readuint32(), s.FighterIndex, s.OnFighterIndexUpdate, shouldNotify)
//     case 1: s.Stacks = s.MaybeNotify(1, reader.Readuint32(), s.Stacks, s.OnStacksUpdate, shouldNotify)
//     case 2: s.Apply = s.MaybeNotify(2, VectorDeserialize(reader, s.Path.GetNested(2)), s.Apply, s.OnApplyUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionApplyRage) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionApplyRage) ReplayListPop() { panic("") }
// func (s *ActionApplyRage) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionApplyRage) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (29): ChimeraAction
// type ActionChimeraAction struct {
//     Path *Path
//     OnFighterIndexUpdate *EventHandler
//     OnChimeraAbilityUpdate *EventHandler
//     OnBeforeChimeraActionUpdate *EventHandler
//     OnChimeraActionUpdate *EventHandler
//     OnAfterChimeraActionUpdate *EventHandler
//     FighterIndex uint32
//     ChimeraAbility ChimeraAbility
//     BeforeChimeraAction Vector
//     ChimeraAction Vector
//     AfterChimeraAction Vector
// }

// func NewActionChimeraAction(path *Path, tag uint32) ActionChimeraAction {
//     obj := ActionChimeraAction{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.ChimeraAbility = NewChimeraAbility(obj.Path.GetNested(1), 0)
//     obj.BeforeChimeraAction = NewVector(obj.Path.GetNested(2), 0)
//     obj.ChimeraAction = NewVector(obj.Path.GetNested(3), 0)
//     obj.AfterChimeraAction = NewVector(obj.Path.GetNested(4), 0)
//     return obj
// }

// func (s *ActionChimeraAction) ClearFighterIndexUpdateHandlers() { s.OnFighterIndexUpdate = nil }
// func (s *ActionChimeraAction) ClearChimeraAbilityUpdateHandlers() { s.OnChimeraAbilityUpdate = nil }
// func (s *ActionChimeraAction) ClearBeforeChimeraActionUpdateHandlers() { s.OnBeforeChimeraActionUpdate = nil }
// func (s *ActionChimeraAction) ClearChimeraActionUpdateHandlers() { s.OnChimeraActionUpdate = nil }
// func (s *ActionChimeraAction) ClearAfterChimeraActionUpdateHandlers() { s.OnAfterChimeraActionUpdate = nil }

// func (s *ActionChimeraAction) ClearUpdateHandlers() {
//     s.OnFighterIndexUpdate = nil
//     s.OnChimeraAbilityUpdate = nil
//     s.OnBeforeChimeraActionUpdate = nil
//     s.OnChimeraActionUpdate = nil
//     s.OnAfterChimeraActionUpdate = nil}

// func ActionChimeraActionDeserialize(_type reflect.Type, reader IReader, path *Path) ActionChimeraAction {
//     chimeraAction := NewActionChimeraAction(path, 0)
//     chimeraAction.Replace(reader, false)
//     return chimeraAction
// }

// func (s *ActionChimeraAction) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Sized
//     case 3: return &WireType.Sized
//     case 4: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionChimeraAction) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.ChimeraAbility
//     case 2: return &s.BeforeChimeraAction
//     case 3: return &s.ChimeraAction
//     case 4: return &s.AfterChimeraAction
//     default: return nil
//     }
// }

// func (s *ActionChimeraAction) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.FighterIndex = s.MaybeNotify(0, reader.Readuint32(), s.FighterIndex, s.OnFighterIndexUpdate, shouldNotify)
//     case 1: s.ChimeraAbility = s.MaybeNotify(1, ChimeraAbilityDeserialize(reader, s.Path.GetNested(1)), s.ChimeraAbility, s.OnChimeraAbilityUpdate, shouldNotify)
//     case 2: s.BeforeChimeraAction = s.MaybeNotify(2, VectorDeserialize(reader, s.Path.GetNested(2)), s.BeforeChimeraAction, s.OnBeforeChimeraActionUpdate, shouldNotify)
//     case 3: s.ChimeraAction = s.MaybeNotify(3, VectorDeserialize(reader, s.Path.GetNested(3)), s.ChimeraAction, s.OnChimeraActionUpdate, shouldNotify)
//     case 4: s.AfterChimeraAction = s.MaybeNotify(4, VectorDeserialize(reader, s.Path.GetNested(4)), s.AfterChimeraAction, s.OnAfterChimeraActionUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionChimeraAction) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionChimeraAction) ReplayListPop() { panic("") }
// func (s *ActionChimeraAction) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionChimeraAction) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (30): Endgame
// type ActionEndgame struct {
//     Path *Path
//     OnWinnerUpdate *EventHandler
//     OnEndgameUpdate *EventHandler
//     Winner uint32
//     Endgame Vector
// }

// func NewActionEndgame(path *Path, tag uint32) ActionEndgame {
//     obj := ActionEndgame{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.Endgame = NewVector(obj.Path.GetNested(1), 0)
//     return obj
// }

// func (s *ActionEndgame) ClearWinnerUpdateHandlers() { s.OnWinnerUpdate = nil }
// func (s *ActionEndgame) ClearEndgameUpdateHandlers() { s.OnEndgameUpdate = nil }

// func (s *ActionEndgame) ClearUpdateHandlers() {
//     s.OnWinnerUpdate = nil
//     s.OnEndgameUpdate = nil}

// func ActionEndgameDeserialize(_type reflect.Type, reader IReader, path *Path) ActionEndgame {
//     endgame := NewActionEndgame(path, 0)
//     endgame.Replace(reader, false)
//     return endgame
// }

// func (s *ActionEndgame) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionEndgame) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.Endgame
//     default: return nil
//     }
// }

// func (s *ActionEndgame) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.Winner = s.MaybeNotify(0, reader.Readuint32(), s.Winner, s.OnWinnerUpdate, shouldNotify)
//     case 1: s.Endgame = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.Endgame, s.OnEndgameUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionEndgame) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionEndgame) ReplayListPop() { panic("") }
// func (s *ActionEndgame) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionEndgame) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (31): EndTurn
// type ActionEndTurn struct {
//     Path *Path
//     OnNextTurnPlayerIndexUpdate *EventHandler
//     OnEndTurnUpdate *EventHandler
//     NextTurnPlayerIndex uint32
//     EndTurn Vector
// }

// func NewActionEndTurn(path *Path, tag uint32) ActionEndTurn {
//     obj := ActionEndTurn{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.EndTurn = NewVector(obj.Path.GetNested(1), 0)
//     return obj
// }

// func (s *ActionEndTurn) ClearNextTurnPlayerIndexUpdateHandlers() { s.OnNextTurnPlayerIndexUpdate = nil }
// func (s *ActionEndTurn) ClearEndTurnUpdateHandlers() { s.OnEndTurnUpdate = nil }

// func (s *ActionEndTurn) ClearUpdateHandlers() {
//     s.OnNextTurnPlayerIndexUpdate = nil
//     s.OnEndTurnUpdate = nil}

// func ActionEndTurnDeserialize(_type reflect.Type, reader IReader, path *Path) ActionEndTurn {
//     endTurn := NewActionEndTurn(path, 0)
//     endTurn.Replace(reader, false)
//     return endTurn
// }

// func (s *ActionEndTurn) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionEndTurn) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.EndTurn
//     default: return nil
//     }
// }

// func (s *ActionEndTurn) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.NextTurnPlayerIndex = s.MaybeNotify(0, reader.Readuint32(), s.NextTurnPlayerIndex, s.OnNextTurnPlayerIndexUpdate, shouldNotify)
//     case 1: s.EndTurn = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.EndTurn, s.OnEndTurnUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionEndTurn) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionEndTurn) ReplayListPop() { panic("") }
// func (s *ActionEndTurn) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionEndTurn) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (32): UpdateChimeraIntents
// type ActionUpdateChimeraIntents struct {
//     Path *Path
//     OnUpdateUpdate *EventHandler
//     Update Vector
// }

// func NewActionUpdateChimeraIntents(path *Path, tag uint32) ActionUpdateChimeraIntents {
//     obj := ActionUpdateChimeraIntents{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.Update = NewVector(obj.Path.GetNested(0), 0)
//     return obj
// }

// func (s *ActionUpdateChimeraIntents) ClearUpdateUpdateHandlers() {
//     s.OnUpdateUpdate = nil
// }

// func (s *ActionUpdateChimeraIntents) ClearUpdateHandlers() {
//     s.OnUpdateUpdate = nil}

// func ActionUpdateChimeraIntentsDeserialize(_type reflect.Type, reader IReader, path *Path) ActionUpdateChimeraIntents {
//     updateChimeraIntents := NewActionUpdateChimeraIntents(path, 0)
//     updateChimeraIntents.Replace(reader, false)
//     return updateChimeraIntents
// }

// func (s *ActionUpdateChimeraIntents) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionUpdateChimeraIntents) GetNested(tag uint32) *IState {
//     switch tag {
//     case 0: return &s.Update
//     default: return nil
//     }
// }

// func (s *ActionUpdateChimeraIntents) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.Update = s.MaybeNotify(0, VectorDeserialize(reader, s.Path.GetNested(0)), s.Update, s.OnUpdateUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionUpdateChimeraIntents) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionUpdateChimeraIntents) ReplayListPop() { panic("") }
// func (s *ActionUpdateChimeraIntents) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionUpdateChimeraIntents) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (33): ChangeChimeraWave
// type ActionChangeChimeraWave struct {
//     Path *Path
//     OnChimeraWaveNumberUpdate *EventHandler
//     OnChangeUpdate *EventHandler
//     ChimeraWaveNumber uint32
//     Change Vector
// }

// func NewActionChangeChimeraWave(path *Path, tag uint32) ActionChangeChimeraWave {
//     obj := ActionChangeChimeraWave{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.Change = NewVector(obj.Path.GetNested(1), 0)
//     return obj
// }

// func (s *ActionChangeChimeraWave) ClearChimeraWaveNumberUpdateHandlers() { s.OnChimeraWaveNumberUpdate = nil }
// func (s *ActionChangeChimeraWave) ClearChangeUpdateHandlers() { s.OnChangeUpdate = nil }

// func (s *ActionChangeChimeraWave) ClearUpdateHandlers() {
//     s.OnChimeraWaveNumberUpdate = nil
//     s.OnChangeUpdate = nil}

// func ActionChangeChimeraWaveDeserialize(_type reflect.Type, reader IReader, path *Path) ActionChangeChimeraWave {
//     changeChimeraWave := NewActionChangeChimeraWave(path, 0)
//     changeChimeraWave.Replace(reader, false)
//     return changeChimeraWave
// }

// func (s *ActionChangeChimeraWave) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     default: return nil
//     }
// }

// func (s *ActionChangeChimeraWave) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.Change
//     default: return nil
//     }
// }

// func (s *ActionChangeChimeraWave) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.ChimeraWaveNumber = s.MaybeNotify(0, reader.Readuint32(), s.ChimeraWaveNumber, s.OnChimeraWaveNumberUpdate, shouldNotify)
//     case 1: s.Change = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.Change, s.OnChangeUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionChangeChimeraWave) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionChangeChimeraWave) ReplayListPop() { panic("") }
// func (s *ActionChangeChimeraWave) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionChangeChimeraWave) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }

// // Variant (34): DeadFighterCardTransform
// type ActionDeadFighterCardTransform struct {
//     Path *Path
//     OnFromCardIdUpdate *EventHandler
//     OnTransformUpdate *EventHandler
//     OnToCardIdUpdate *EventHandler
//     FromCardId uint32
//     Transform Vector
//     ToCardId uint32
// }

// func NewActionDeadFighterCardTransform(path *Path, tag uint32) ActionDeadFighterCardTransform {
//     obj := ActionDeadFighterCardTransform{}

//     if path != nil {
//         obj.Path = path
//     } else {
//         obj.Path = Path.Root
//     }
//     obj.Transform = NewVector(obj.Path.GetNested(1), 0)
//     return obj
// }

// func (s *ActionDeadFighterCardTransform) ClearFromCardIdUpdateHandlers() { s.OnFromCardIdUpdate = nil }
// func (s *ActionDeadFighterCardTransform) ClearTransformUpdateHandlers() { s.OnTransformUpdate = nil }
// func (s *ActionDeadFighterCardTransform) ClearToCardIdUpdateHandlers() { s.OnToCardIdUpdate = nil }

// func (s *ActionDeadFighterCardTransform) ClearUpdateHandlers() {
//     s.OnFromCardIdUpdate = nil
//     s.OnTransformUpdate = nil
//     s.OnToCardIdUpdate = nil}

// func ActionDeadFighterCardTransformDeserialize(_type reflect.Type, reader IReader, path *Path) ActionDeadFighterCardTransform {
//     deadFighterCardTransform := NewActionDeadFighterCardTransform(path, 0)
//     deadFighterCardTransform.Replace(reader, false)
//     return deadFighterCardTransform
// }

// func (s *ActionDeadFighterCardTransform) GetWireType(tag uint32) *WireType {
//     switch tag {
//     case 0: return &WireType.Varint
//     case 1: return &WireType.Sized
//     case 2: return &WireType.Varint
//     default: return nil
//     }
// }

// func (s *ActionDeadFighterCardTransform) GetNested(tag uint32) *IState {
//     switch tag {
//     case 1: return &s.Transform
//     default: return nil
//     }
// }

// func (s *ActionDeadFighterCardTransform) ReplaceAt(tag uint32,wireType WireType,reader IReader,shouldNotify bool) {
//     switch tag {
//     case 0: s.FromCardId = s.MaybeNotify(0, reader.Readuint32(), s.FromCardId, s.OnFromCardIdUpdate, shouldNotify)
//     case 1: s.Transform = s.MaybeNotify(1, VectorDeserialize(reader, s.Path.GetNested(1)), s.Transform, s.OnTransformUpdate, shouldNotify)
//     case 2: s.ToCardId = s.MaybeNotify(2, reader.Readuint32(), s.ToCardId, s.OnToCardIdUpdate, shouldNotify)
//     default: reader.SkipField(wireType)
//     }
// }

// func (s *ActionDeadFighterCardTransform) ReplayListPush(reader IReader) { panic("") }
// func (s *ActionDeadFighterCardTransform) ReplayListPop() { panic("") }
// func (s *ActionDeadFighterCardTransform) ReplayMapRemove(key uint32) { panic("") }

// func (s *ActionDeadFighterCardTransform) MaybeNotify(
//     tag uint32,
//     newValue interface{},
//     oldValue interface{},
//     handler EventHandler,
//     shouldNotify bool,
// ) interface{} {
//     if shouldNotify {
//         args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
//         handler.Invoke(s, args)
//     }

//     return newValue
// }
