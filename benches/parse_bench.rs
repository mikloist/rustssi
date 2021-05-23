use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

pub use rustssi::types::messages;
use rustssi::types::messages::GenericMessage;

pub fn criterion_benchmark(c: &mut Criterion) {
    let strings = vec![
        ":WiZ!jto@tolsun.oulu.fi TOPIC #test :New topic ",
        ":Angel!wings@irc.org PRIVMSG Wiz :Are you receiving this message ?",
        "PRIVMSG Angel :yes I'm receiving it !",
        "PRIVMSG jto@tolsun.oulu.fi :Hello !",
        "PRIVMSG kalt%millennium.stealth.net@irc.stealth.net :Are you a frog?",
        "PRIVMSG kalt%millennium.stealth.net :Do you like cheese?",
        "PRIVMSG Wiz!jto@tolsun.oulu.fi :Hello !",
        "PRIVMSG $*.fi :Server tolsun.oulu.fi rebooting.",
        "PRIVMSG #*.edu :NSFNet is undergoing work, expect interruptions",
        "MODE #Finnish +imI *!*@*.fi",
        "MODE #Finnish +o Kilroy",
        "MODE #Finnish +v Wiz",
        ":WiZ!jto@tolsun.oulu.fi PART #playzone :I lost",
        ":syrk!kalt@millennium.stealth.net QUIT :Gone to have lunch",
        "800 #Finnish +v Wiz",
        ":syrk!kalt@millennium.stealth.net QUIT kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong :Gone to have lunch",
        ":syrk!kalt@millennium.stealth.net QUIT kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  :Gone to have lunch",
        ":syrk!kalt@millennium.stealth.net QUIT kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net :Gone to have lunch",
        ":syrk!kalt@millennium.stealth.net QUIT kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net kalt%millennium.stealth.net@irc.stealth.net :Gone to have lunch",
        ":syrk!kalt@millennium.stealth.net QUIT kalt%millennium.stealth.net@irc.stealth.net kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net kalt%millennium.stealth.net@irc.stealth.net :Gone to have lunch",
        ":syrk!kalt@millennium.stealth.net QUIT kalt%millennium.stealth.net@irc.stealth.net kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net loooooooooooooooooooooooooooooooooooooooooooooooooong  kalt%millennium.stealth.net@irc.stealth.net kalt%net@irc.stealth.net :Gone to have lunch",
    ];

    let mut group = c.benchmark_group("msg_parsing");
    for (elem, &str) in strings.iter().enumerate() {
        group.throughput(Throughput::Elements(str.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(elem), &str, |b, &str| {
            b.iter(|| GenericMessage::from_bytes(str.as_bytes()))
        });
    }
    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
