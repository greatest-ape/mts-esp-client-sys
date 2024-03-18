#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! (MTS-ESP)[https://github.com/ODDSound/MTS-ESP/tree/main] client bindings

/// Opaque datatype for MTSClient.
#[repr(C)]
pub struct MTSClient {
    _unused: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    /// Register as a client. Call from the plugin constructor.
    pub fn MTS_RegisterClient() -> *mut MTSClient;
    /// Deregister as a client. Call from the plugin destructor.
    pub fn MTS_DeregisterClient(client: *mut MTSClient);

    /// Check if the client is currently connected to a master plugin.
    pub fn MTS_HasMaster(client: *mut MTSClient) -> bool;

    /// Returns true if note should not be played. MIDI channel argument should be included if possible (0-15), else set to -1.
    pub fn MTS_ShouldFilterNote(
        client: *mut MTSClient,
        midinote: ::std::os::raw::c_char,
        midichannel: ::std::os::raw::c_char,
    ) -> bool;

    /// Retuning a midi note. Pick the version that makes your life easiest! MIDI channel argument should be included if possible (0-15), else set to -1.
    pub fn MTS_NoteToFrequency(
        client: *mut MTSClient,
        midinote: ::std::os::raw::c_char,
        midichannel: ::std::os::raw::c_char,
    ) -> f64;
    /// Retuning a midi note. Pick the version that makes your life easiest! MIDI channel argument should be included if possible (0-15), else set to -1.
    pub fn MTS_RetuningInSemitones(
        client: *mut MTSClient,
        midinote: ::std::os::raw::c_char,
        midichannel: ::std::os::raw::c_char,
    ) -> f64;
    /// Retuning a midi note. Pick the version that makes your life easiest! MIDI channel argument should be included if possible (0-15), else set to -1.
    pub fn MTS_RetuningAsRatio(
        client: *mut MTSClient,
        midinote: ::std::os::raw::c_char,
        midichannel: ::std::os::raw::c_char,
    ) -> f64;

    /// MTS_FrequencyToNote() is a helper function returning the note number whose pitch is closest to the supplied frequency. Two versions are provided:
    /// The first is for the simplest case: supply a frequency and get a note number back.
    /// If you intend to use the returned note number to generate a note-on message on a specific, pre-determined MIDI channel, set the midichannel argument to the destination channel (0-15), else set to -1.
    /// If a MIDI channel is supplied, the corresponding multi-channel tuning table will be queried if in use, else multi-channel tables are ignored.
    pub fn MTS_FrequencyToNote(
        client: *mut MTSClient,
        freq: f64,
        midichannel: ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_char;

    /// Use the second version if you intend to use the returned note number to generate a note-on message and where you have the possibility to send it on any MIDI channel.
    /// The midichannel argument is a pointer to a char which will receive the MIDI channel on which the note message should be sent (0-15).
    /// Multi-channel tuning tables are queried if in use.
    pub fn MTS_FrequencyToNoteAndChannel(
        client: *mut MTSClient,
        freq: f64,
        midichannel: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_char;

    /// Returns the name of the current scale.
    pub fn MTS_GetScaleName(client: *mut MTSClient) -> *const ::std::os::raw::c_char;

    /// Parse incoming MIDI data to update local retuning. All formats of MTS sysex message accepted.
    pub fn MTS_ParseMIDIDataU(
        client: *mut MTSClient,
        buffer: *const ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_int,
    );
    /// Parse incoming MIDI data to update local retuning. All formats of MTS sysex message accepted.
    pub fn MTS_ParseMIDIData(
        client: *mut MTSClient,
        buffer: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    );
}
