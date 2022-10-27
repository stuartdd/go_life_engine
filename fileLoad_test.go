package go_life

import (
	"fmt"
	"testing"
)

func TestFileSave(t *testing.T) {
	rle, err := NewRleFile("testdata/rats.rle")
	if err != nil {
		t.Errorf("RLE File load failed. %e", err)
	}

	rleSave := NewRLESave("testdata/ab", rle.coords, "OWNER", "DESC")
	s := rleSave.SaveFileContent()
	fmt.Println(s)
}

func TestFileEncode(t *testing.T) {
	rle, err := NewRleFile("testdata/rats.rle")
	if err != nil {
		t.Errorf("RLE File load failed. %e", err)
	}
	enc, w, h := rle.Encode()
	if rle.encoded != enc {
		t.Errorf("RLE File Encode failed. \n%s\n%s", rle.encoded, enc)
	}
	if w != 12 {
		t.Errorf("RLE File Encode failed. Expected width %d Actual Width %d", 12, w)
	}
	if h != 11 {
		t.Errorf("RLE File Encode failed. Expected height %d Actual Width %d", 11, h)
	}

	saveRle := NewRLESave("RLESave", rle.coords, "owner", "desc")
	if rle.decoded != saveRle.decoded {
		t.Errorf("RLE File Encode failed. Decoded expected \n%s Actual \n%s", rle.decoded, saveRle.decoded)
	}
	if rle.encoded != saveRle.encoded {
		t.Errorf("RLE File Encode failed. Encoded expected \n%s Actual \n%s", rle.encoded, saveRle.encoded)
	}
}
