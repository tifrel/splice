use assert_cmd::Command;

macro_rules! call_with_args {
    ( $( $arg:expr ),* ) => {
        {
            Command::cargo_bin("splice").unwrap()
            $(
                .arg($arg)
            )*.assert()
        }
    };
}

const TESTFILE: &str = "tests/testfile";

#[test]
fn failures() {
    // too few args
    call_with_args!().failure();
    call_with_args!("id").failure();
    // incompatible flags
    call_with_args!("-e", "-i", "ins", "id", "fname").failure();
}

#[test]
fn inserts() {
    call_with_args!("-i", "lol", "intron1", TESTFILE).stdout(
        "this is line 1 (pre-exon)\n\
        this is line 2 (pre-exon)\n\
        # spliceSTART intron1\n\
        lol\n\
        # spliceSTOP intron1\n\
        this is line 6 (inter-exon)\n\
        # spliceSTART intron2 and gibberish\n\
        line inside intron2\n\
        # spliceSTOP intron2\n\
        this is line 10 (post-exon)\n",
    );
}

#[test]
fn intron_prints() {
    // normal intron
    call_with_args!("intron1", TESTFILE).stdout(
        "# spliceSTART intron1\n\
        line inside intron1\n\
        # spliceSTOP intron1\n",
    );

    // intron with comment
    call_with_args!("intron2", TESTFILE).stdout(
        "# spliceSTART intron2 and gibberish\n\
        line inside intron2\n\
        # spliceSTOP intron2\n",
    );
}

#[test]
fn exon_prints() {
    // normal intron
    call_with_args!("-e", "intron1", TESTFILE).stdout(
        "this is line 1 (pre-exon)\n\
        this is line 2 (pre-exon)\n\
        # spliceSTART intron1\n\
        # spliceSTOP intron1\n\
        this is line 6 (inter-exon)\n\
        # spliceSTART intron2 and gibberish\n\
        line inside intron2\n\
        # spliceSTOP intron2\n\
        this is line 10 (post-exon)\n",
    );

    // intron with comment
    call_with_args!("-e", "intron2", TESTFILE).stdout(
        "this is line 1 (pre-exon)\n\
        this is line 2 (pre-exon)\n\
        # spliceSTART intron1\n\
        line inside intron1\n\
        # spliceSTOP intron1\n\
        this is line 6 (inter-exon)\n\
        # spliceSTART intron2 and gibberish\n\
        # spliceSTOP intron2\n\
        this is line 10 (post-exon)\n",
    );
}

// unedited testfile for copy pasta:
//      "this is line 1 (pre-exon)\n\
//      this is line 2 (pre-exon)\n\
//      # spliceSTART intron1\n\
//      line inside intron1\n\
//      # spliceSTOP intron1\n\
//      this is line 6 (inter-exon)\n\
//      # spliceSTART intron2 and gibberish\n\
//      line inside intron2\n\
//      # spliceSTOP intron2\n\
//      this is line 10 (post-exon)\n",
