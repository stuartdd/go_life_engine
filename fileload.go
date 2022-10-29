package go_life

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"path"
	"strconv"
	"strings"
	"time"
	"unicode"
)

type RLE struct {
	fileName string
	decoded  string
	coords   []int32
	encoded  string
	name     string
	owner    string
	comment  string
	minX     int32
	minY     int32
	maxX     int32
	maxY     int32
}

func NewRLESave(fn string, coords []int32, owner, comment string) *RLE {
	rle := &RLE{fileName: fn, coords: coords, owner: owner, comment: comment}
	fnlc := strings.ToLower(fn)
	ext := path.Ext(fnlc)
	if ext != ".rle" {
		fn = fn[:len(fn)-len(ext)]
		fn = fn + ".rle"
	}
	_, rle.name = path.Split(fn)
	rle.fileName = fn
	enc, _, _ := rle.Encode()
	rle.encoded = enc
	rle.decoded, _ = rle.rleDecodeString(rle.encoded)
	return rle
}

func (rle *RLE) Save() error {
	file, err := os.Create(rle.fileName)
	if err != nil {
		return err
	}
	defer file.Close()
	_, err = file.WriteString(rle.SaveFileContent())
	return err
}

func (rle *RLE) SaveFileContent() string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("#N %s\n", rle.name))
	sb.WriteString(fmt.Sprintf("#O %s\n", rle.owner))
	sb.WriteString(fmt.Sprintf("#C Created: %s\n", time.Now().Format("Monday January 2 2006")))
	sb.WriteString(fmt.Sprintf("#C %s\n", rle.comment))
	sb.WriteString("x = 0, y = 0, rule = B3/S23\n")
	sb.WriteString(rle.encoded)
	return sb.String()
}

func NewRleFile(fileName string) (*RLE, error) {
	rle := &RLE{fileName: fileName}
	file, err := os.Open(rle.fileName)
	if err != nil {
		return nil, err
	}
	scanner := bufio.NewScanner(file)
	var sb strings.Builder
	ln := 0
	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "#N") {
			rle.name = strings.TrimSpace(line[2:])
		} else {
			if strings.HasPrefix(line, "#C") {
				if rle.comment == "" {
					rle.comment = strings.TrimSpace(line[2:])
				}
			} else {
				if strings.HasPrefix(line, "#O") {
					rle.owner = strings.TrimSpace(line[2:])
				} else {
					if !strings.HasPrefix(line, "#") {
						if ln > 0 {
							sb.WriteString(line)
						}
						ln++
					}
				}
			}
		}
	}
	rle.encoded = sb.String()
	rle.decoded, rle.coords = rle.rleDecodeString(sb.String())
	if len(rle.coords) == 0 {
		rle.minX = 0
		rle.minY = 0
		rle.maxX = 0
		rle.maxY = 0
	} else {
		rle.minX = math.MaxInt32
		rle.minY = math.MaxInt32
		rle.maxX = math.MinInt32
		rle.maxY = math.MinInt32
	}
	for i := 0; i < len(rle.coords); i = i + 2 {
		if rle.coords[i] < rle.minX {
			rle.minX = rle.coords[i]
		}
		if rle.coords[i] > rle.maxX {
			rle.maxX = rle.coords[i]
		}
		if rle.coords[i+1] < rle.minY {
			rle.minY = rle.coords[i+1]
		}
		if rle.coords[i+1] > rle.maxY {
			rle.maxY = rle.coords[i+1]
		}
	}
	if scanner.Err() != nil {
		return nil, scanner.Err()
	}
	return rle, nil
}

func (rle *RLE) Filename() string {
	return rle.fileName
}

func (rle *RLE) Comment() string {
	return rle.comment
}

func (rle *RLE) Coords() []int32 {
	return rle.coords
}

func (rle *RLE) Center() (int32, int32) {
	if len(rle.coords) == 0 {
		return 0, 0
	}
	return (rle.maxX - rle.minX) / 2, (rle.maxY - rle.minY) / 2
}

func (rle *RLE) rleDecodeString(rleStr string) (string, []int32) {
	var result strings.Builder
	coords := make([]int32, 0)
	for len(rleStr) > 0 {
		letterIndex := strings.IndexFunc(rleStr, func(r rune) bool { return !unicode.IsDigit(r) })
		multiply := 1
		if letterIndex != 0 {
			multiply, _ = strconv.Atoi(rleStr[:letterIndex])
		}
		result.WriteString(strings.Repeat(string(rleStr[letterIndex]), multiply))
		rleStr = rleStr[letterIndex+1:]
	}
	out := result.String()

	var sb strings.Builder
	count := 0
	width := 0
	var y int32 = 0
	var x int32 = 0
	for _, c := range out {
		switch c {
		case '$':
			y++
			x = 0
			if count == 0 {
				for i := 0; i <= width; i++ {
					sb.WriteString("| ")
				}
				sb.WriteString("\n")
			} else {
				sb.WriteString("|\n")
				width = count
			}
			count = 0
		case 'b':
			sb.WriteString("| ")
			count++
			x++
		case 'o':
			coords = append(coords, x)
			coords = append(coords, y)
			sb.WriteString("|O")
			count++
			x++
		case '!':
			for i := 0; i <= (width - count); i++ {
				sb.WriteString("| ")
			}
		}
	}
	rle.maxX = x
	rle.minY = y
	return sb.String(), coords
}

func (rle *RLE) Encode() (string, int32, int32) {
	return RLEEncodeCoords(rle.coords)
}

func RLEEncodeCoords(coords []int32) (string, int32, int32) {
	co, w, h := POCNormaliseCoords(coords)
	var enc strings.Builder
	for y := 0; y < int(h); y++ {
		countOn := 0
		countOff := 0
		for x := 0; x < int(w); x++ {
			found := false
			for i := 0; i < len(co); i = i + 2 {
				if x == int(co[i]) && y == int(co[i+1]) {
					found = true
					break
				}
			}
			if found {
				countOff = rleEncodeAppend(&enc, countOff, false)
				countOn++
			} else {
				countOn = rleEncodeAppend(&enc, countOn, true)
				countOff++
			}
		}
		if y < int(h-1) {
			rleEncodeAppend(&enc, countOff, false)
			rleEncodeAppend(&enc, countOn, true)
			enc.WriteString("$")
		} else {
			rleEncodeAppend(&enc, countOn, true)
			enc.WriteString("!")
		}
	}
	return enc.String(), w, h

}

func rleEncodeAppend(enc *strings.Builder, n int, on bool) int {
	if n > 0 {
		c := "b"
		if on {
			c = "o"
		}
		if n == 1 {
			enc.WriteString(c)
		} else {
			enc.WriteString(fmt.Sprintf("%d%s", n, c))
		}
	}
	return 0
}

func (rle *RLE) String() string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Owner  :%s\n", rle.owner))
	sb.WriteString(fmt.Sprintf("Name   :%s\n", rle.name))
	sb.WriteString(fmt.Sprintf("File   :%s\n", rle.fileName))
	sb.WriteString(fmt.Sprintf("Comment:%s\n", rle.comment))
	sb.WriteString(fmt.Sprintf("Encoded:%s\n", rle.encoded))
	sb.WriteString(fmt.Sprintf("Cells  :%d\n", len(rle.coords)/2))
	sb.WriteString(fmt.Sprintf("x,y    :%d, %d\n", rle.minX, rle.minY))

	for i := 0; i < len(rle.coords); i = i + 2 {
		sb.WriteString(fmt.Sprintf("%3d, %3d ", rle.coords[i], rle.coords[i+1]))
	}
	sb.WriteString("\n")
	sb.WriteString(fmt.Sprintf("%s\n", rle.decoded))
	return sb.String()
}

func POCNormaliseCoords(in []int32) ([]int32, int32, int32) {
	out := make([]int32, len(in))
	minx := int32(math.MaxInt32)
	miny := int32(math.MaxInt32)
	maxx := int32(math.MinInt32)
	maxy := int32(math.MinInt32)

	for i := 0; i < len(in); i = i + 2 {
		if in[i] < minx {
			minx = in[i]
		}
		if in[i] > maxx {
			maxx = in[i]
		}
		if in[i+1] < miny {
			miny = in[i+1]
		}
		if in[i+1] > maxy {
			maxy = in[i+1]
		}
	}
	for i := 0; i < len(in); i = i + 2 {
		out[i] = in[i] - minx
		out[i+1] = in[i+1] - miny
	}
	return out, (maxx - minx) + 1, (maxy - miny) + 1
}
