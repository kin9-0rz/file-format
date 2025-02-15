use file_format::FileFormat;

#[test]
fn test_aac() {
    let format = FileFormat::from_file("fixtures/audio/sample.aac").unwrap();
    assert_eq!(format, FileFormat::AdvancedAudioCoding);
}

#[test]
fn test_ac3() {
    let format = FileFormat::from_file("fixtures/audio/sample.ac3").unwrap();
    assert_eq!(format, FileFormat::AudioCodec3);
}

#[test]
fn test_aiff() {
    let format = FileFormat::from_file("fixtures/audio/sample.aiff").unwrap();
    assert_eq!(format, FileFormat::AudioInterchangeFileFormat);
}

#[test]
fn test_amr() {
    let format = FileFormat::from_file("fixtures/audio/sample.amr").unwrap();
    assert_eq!(format, FileFormat::AdaptiveMultiRate);
}

#[test]
fn test_ape() {
    let format = FileFormat::from_file("fixtures/audio/sample.ape").unwrap();
    assert_eq!(format, FileFormat::MonkeysAudio);
}

#[test]
fn test_au() {
    let format = FileFormat::from_file("fixtures/audio/sample.au").unwrap();
    assert_eq!(format, FileFormat::Au);
}

#[test]
fn test_dsf() {
    let format = FileFormat::from_file("fixtures/audio/sample.dsf").unwrap();
    assert_eq!(format, FileFormat::SonyDsdStreamFile);
}

#[test]
fn test_f4a() {
    let format = FileFormat::from_file("fixtures/audio/sample.f4a").unwrap();
    assert_eq!(format, FileFormat::AdobeFlashPlayerAudio);
}

#[test]
fn test_f4b() {
    let format = FileFormat::from_file("fixtures/audio/sample.f4b").unwrap();
    assert_eq!(format, FileFormat::AdobeFlashPlayerAudiobook);
}

#[test]
fn test_flac() {
    let format = FileFormat::from_file("fixtures/audio/sample.flac").unwrap();
    assert_eq!(format, FileFormat::FreeLosslessAudioCodec);
}

#[test]
fn test_it() {
    let format = FileFormat::from_file("fixtures/audio/sample.it").unwrap();
    assert_eq!(format, FileFormat::ImpulseTrackerModule);
}

#[test]
fn test_m4a() {
    let format = FileFormat::from_file("fixtures/audio/sample.m4a").unwrap();
    assert_eq!(format, FileFormat::AppleItunesAudio);
}

#[test]
fn test_m4b() {
    let format = FileFormat::from_file("fixtures/audio/sample.m4b").unwrap();
    assert_eq!(format, FileFormat::AppleItunesAudiobook);
}

#[test]
fn test_m4p() {
    let format = FileFormat::from_file("fixtures/audio/sample.m4p").unwrap();
    assert_eq!(format, FileFormat::AppleItunesProtectedAudio);
}

#[test]
fn test_mid() {
    let format = FileFormat::from_file("fixtures/audio/sample.mid").unwrap();
    assert_eq!(format, FileFormat::MusicalInstrumentDigitalInterface);
}

#[test]
fn test_mp3() {
    let format = FileFormat::from_file("fixtures/audio/sample.mp3").unwrap();
    assert_eq!(format, FileFormat::MpegAudioLayer3);
}

#[test]
fn test_mpc() {
    let format = FileFormat::from_file("fixtures/audio/sample.mpc").unwrap();
    assert_eq!(format, FileFormat::Musepack);
}

#[test]
fn test_oga() {
    let format = FileFormat::from_file("fixtures/audio/sample.oga").unwrap();
    assert_eq!(format, FileFormat::OggFlac);
}

#[test]
fn test_ogg() {
    let format = FileFormat::from_file("fixtures/audio/sample.ogg").unwrap();
    assert_eq!(format, FileFormat::OggVorbis);
}

#[test]
fn test_opus() {
    let format = FileFormat::from_file("fixtures/audio/sample.opus").unwrap();
    assert_eq!(format, FileFormat::OggOpus);
}

#[test]
fn test_qcp() {
    let format = FileFormat::from_file("fixtures/audio/sample.qcp").unwrap();
    assert_eq!(format, FileFormat::QualcommPureVoice);
}

#[test]
fn test_s3m() {
    let format = FileFormat::from_file("fixtures/audio/sample.s3m").unwrap();
    assert_eq!(format, FileFormat::ScreamTracker3Module);
}

#[test]
fn test_spx() {
    let format = FileFormat::from_file("fixtures/audio/sample.spx").unwrap();
    assert_eq!(format, FileFormat::OggSpeex);
}

#[test]
fn test_wav() {
    let format = FileFormat::from_file("fixtures/audio/sample.wav").unwrap();
    assert_eq!(format, FileFormat::WaveformAudio);
}

#[test]
fn test_wv() {
    let format = FileFormat::from_file("fixtures/audio/sample.wv").unwrap();
    assert_eq!(format, FileFormat::WavPack);
}

#[test]
fn test_xm() {
    let format = FileFormat::from_file("fixtures/audio/sample.xm").unwrap();
    assert_eq!(format, FileFormat::FastTracker2ExtendedModule);
}
