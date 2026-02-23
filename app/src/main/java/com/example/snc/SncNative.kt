package com.example.snc

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

object SncNative {
    init {
        System.loadLibrary("android_ffi") // libandroid_ffi.so from Rust
    }

    @JvmStatic private external fun distill_chat_turn(inputJson: String): Long
    @JvmStatic private external fun orchestrate_with_hgo_and_shell(
        hgoJson: String,
        contextJson: String,
        sovereigntyStateJson: String,
        disciplineSignalsJson: String,
        fateDeckSeedJson: String
    ): Long
    @JvmStatic private external fun snc_free_string(ptr: Long)

    private fun callAndRead(ptr: Long): String {
        require(ptr != 0L) { "Rust returned null pointer" }
        val bytes = mutableListOf<Byte>()
        var offset = 0L
        while (true) {
            val b = NativeMemory.readByte(ptr + offset) // use NDK/unsafe helper
            if (b.toInt() == 0) break
            bytes.add(b)
            offset += 1
        }
        val s = bytes.toByteArray().toString(Charsets.UTF_8)
        snc_free_string(ptr)
        return s
    }

    suspend fun distillChatTurn(request: DistillRequest): DistillResponse {
        val inputJson = Json.encodeToString(DistillRequest.serializer(), request)
        return kotlinx.coroutines.withContext(kotlinx.coroutines.Dispatchers.IO) {
            val ptr = distill_chat_turn(inputJson)
            val json = callAndRead(ptr)
            Json.decodeFromString(DistillResponse.serializer(), json)
        }
    }

    suspend fun orchestrateWithHgoAndShell(
        hgo: HitGovernanceObjectJson,
        context: AndroidContextJson,
        sovereignty: SovereigntyStateJson,
        discipline: DisciplineSignalsJson,
        fateSeed: FateDeckSeedJson
    ): OrchestrationResponse {
        val hgoJson = Json.encodeToString(HitGovernanceObjectJson.serializer(), hgo)
        val ctxJson = Json.encodeToString(AndroidContextJson.serializer(), context)
        val sovJson = Json.encodeToString(SovereigntyStateJson.serializer(), sovereignty)
        val sigJson = Json.encodeToString(DisciplineSignalsJson.serializer(), discipline)
        val seedJson = Json.encodeToString(FateDeckSeedJson.serializer(), fateSeed)

        return kotlinx.coroutines.withContext(kotlinx.coroutines.Dispatchers.IO) {
            val ptr = orchestrate_with_hgo_and_shell(
                hgoJson,
                ctxJson,
                sovJson,
                sigJson,
                seedJson
            )
            val json = callAndRead(ptr)
            Json.decodeFromString(OrchestrationResponse.serializer(), json)
        }
    }
}
