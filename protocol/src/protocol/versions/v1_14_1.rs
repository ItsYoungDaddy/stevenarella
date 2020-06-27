protocol_packet_ids!(
    handshake Handshaking {
        serverbound Serverbound {
            0x00 => Handshake
        }
        clientbound Clientbound {
        }
    }
    play Play {
        serverbound Serverbound {
            0x00 => TeleportConfirm
            0x01 => QueryBlockNBT
            0x02 => SetDifficulty
            0x03 => ChatMessage
            0x04 => ClientStatus
            0x05 => ClientSettings
            0x06 => TabComplete
            0x07 => ConfirmTransactionServerbound
            0x08 => ClickWindowButton
            0x09 => ClickWindow
            0x0a => CloseWindow
            0x0b => PluginMessageServerbound
            0x0c => EditBook
            0x0d => QueryEntityNBT
            0x0e => UseEntity
            0x0f => KeepAliveServerbound_i64
            0x10 => LockDifficulty
            0x11 => PlayerPosition
            0x12 => PlayerPositionLook
            0x13 => PlayerLook
            0x14 => Player
            0x15 => VehicleMove
            0x16 => SteerBoat
            0x17 => PickItem
            0x18 => CraftRecipeRequest
            0x19 => ClientAbilities_f32
            0x1a => PlayerDigging
            0x1b => PlayerAction
            0x1c => SteerVehicle
            0x1d => CraftingBookData
            0x1e => NameItem
            0x1f => ResourcePackStatus
            0x20 => AdvancementTab
            0x21 => SelectTrade
            0x22 => SetBeaconEffect
            0x23 => HeldItemChange
            0x24 => UpdateCommandBlock
            0x25 => UpdateCommandBlockMinecart
            0x26 => CreativeInventoryAction
            0x27 => UpdateJigsawBlock
            0x28 => UpdateStructureBlock
            0x29 => SetSign
            0x2a => ArmSwing
            0x2b => SpectateTeleport
            0x2c => PlayerBlockPlacement_insideblock
            0x2d => UseItem
        }
        clientbound Clientbound {
            0x00 => SpawnObject_VarInt
            0x01 => SpawnExperienceOrb
            0x02 => SpawnGlobalEntity
            0x03 => SpawnMob_WithMeta
            0x04 => SpawnPainting_VarInt
            0x05 => SpawnPlayer_f64
            0x06 => Animation
            0x07 => Statistics
            0x08 => BlockBreakAnimation
            0x09 => UpdateBlockEntity
            0x0a => BlockAction
            0x0b => BlockChange_VarInt
            0x0c => BossBar
            0x0d => ServerDifficulty_Locked
            0x0e => ServerMessage_Position
            0x0f => MultiBlockChange_VarInt
            0x10 => TabCompleteReply
            0x11 => DeclareCommands
            0x12 => ConfirmTransaction
            0x13 => WindowClose
            0x14 => WindowItems
            0x15 => WindowProperty
            0x16 => WindowSetSlot
            0x17 => SetCooldown
            0x18 => PluginMessageClientbound
            0x19 => NamedSoundEffect
            0x1a => Disconnect
            0x1b => EntityAction
            0x1c => Explosion
            0x1d => ChunkUnload
            0x1e => ChangeGameState
            0x1f => WindowOpenHorse
            0x20 => KeepAliveClientbound_i64
            0x21 => ChunkData_HeightMap
            0x22 => Effect
            0x23 => Particle_Data
            0x24 => UpdateLight
            0x25 => JoinGame_i32_ViewDistance
            0x26 => Maps
            0x27 => TradeList_WithoutRestock
            0x28 => EntityMove_i16
            0x29 => EntityLookAndMove_i16
            0x2a => EntityLook_VarInt
            0x2b => Entity
            0x2c => VehicleTeleport
            0x2d => OpenBook
            0x2e => WindowOpen_VarInt
            0x2f => SignEditorOpen
            0x30 => CraftRecipeResponse
            0x31 => PlayerAbilities
            0x32 => CombatEvent
            0x33 => PlayerInfo
            0x34 => FacePlayer
            0x35 => TeleportPlayer_WithConfirm
            0x36 => UnlockRecipes_WithSmelting
            0x37 => EntityDestroy
            0x38 => EntityRemoveEffect
            0x39 => ResourcePackSend
            0x3a => Respawn_Gamemode
            0x3b => EntityHeadLook
            0x3c => SelectAdvancementTab
            0x3d => WorldBorder
            0x3e => Camera
            0x3f => SetCurrentHotbarSlot
            0x40 => UpdateViewPosition
            0x41 => UpdateViewDistance
            0x42 => ScoreboardDisplay
            0x43 => EntityMetadata
            0x44 => EntityAttach
            0x45 => EntityVelocity
            0x46 => EntityEquipment
            0x47 => SetExperience
            0x48 => UpdateHealth
            0x49 => ScoreboardObjective
            0x4a => SetPassengers
            0x4b => Teams_VarInt
            0x4c => UpdateScore
            0x4d => SpawnPosition
            0x4e => TimeUpdate
            0x4f => Title
            0x50 => EntitySoundEffect
            0x51 => SoundEffect
            0x52 => StopSound
            0x53 => PlayerListHeaderFooter
            0x54 => NBTQueryResponse
            0x55 => CollectItem
            0x56 => EntityTeleport_f64
            0x57 => Advancements
            0x58 => EntityProperties
            0x59 => EntityEffect
            0x5a => DeclareRecipes
            0x5b => TagsWithEntities
        }
    }
    login Login {
        serverbound Serverbound {
            0x00 => LoginStart
            0x01 => EncryptionResponse
            0x02 => LoginPluginResponse
        }
        clientbound Clientbound {
            0x00 => LoginDisconnect
            0x01 => EncryptionRequest
            0x02 => LoginSuccess_String
            0x03 => SetInitialCompression
            0x04 => LoginPluginRequest
        }
    }
    status Status {
        serverbound Serverbound {
            0x00 => StatusRequest
            0x01 => StatusPing
        }
        clientbound Clientbound {
            0x00 => StatusResponse
            0x01 => StatusPong
        }
    }
);
