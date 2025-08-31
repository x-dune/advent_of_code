package main

import (
	"bufio"
	"fmt"
	"os"
)

type Block struct {
	id    int
	start int
	len   int
}

func main() {
	var input string
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		input += scanner.Text()
	}

	files := []Block{}
	spaces := []Block{}

	id := 0
	pos := 0
	for i, ch := range input {
		n := int(ch - '0')
		if n == 0 {
			continue
		}

		if i%2 == 0 {
			files = append(files, Block{
				id:    id,
				start: pos,
				len:   n,
			})
			id++
		} else {
			spaces = append(spaces, Block{
				id:    -1,
				start: pos,
				len:   n,
			})
		}
		pos += n
	}

	fmt.Println(part1(files, pos))
	fmt.Println(part2(files, spaces, pos))
}

func part1(files []Block, diskLen int) int {
	disk := flatten(files, diskLen)
	l := 0
	r := len(disk) - 1

	for l < r {
		for l < r && disk[l] != -1 {
			l++
		}

		for l < r && disk[r] == -1 {
			r--
		}

		if l < r {
			disk[l], disk[r] = disk[r], disk[l]
		}
	}

	return checksum(disk)
}

func flatten(files []Block, diskLen int) []int {
	disk := make([]int, diskLen)
	for i := range disk {
		disk[i] = -1
	}
	for _, f := range files {
		for i := range f.len {
			disk[f.start+i] = f.id
		}
	}
	return disk
}

func checksum(disk []int) int {
	sum := 0
	for i, id := range disk {
		if id == -1 {
			continue
		}
		sum += i * id
	}
	return sum
}

func part2(files []Block, spaces []Block, diskLen int) int {
	for fileIndex := len(files) - 1; fileIndex >= 0; fileIndex-- {
		file := files[fileIndex]

		spaceIndex := -1
		for j, space := range spaces {
			if file.len <= space.len && space.start < file.start {
				spaceIndex = j
				break
			}
		}

		if spaceIndex != -1 {
			space := spaces[spaceIndex]
			files[fileIndex].start = space.start

			if space.len == file.len {
				spaces = append(spaces[:spaceIndex], spaces[spaceIndex+1:]...)
			} else {
				spaces[spaceIndex].start += file.len
				spaces[spaceIndex].len -= file.len
			}
		}
	}

	return checksum(flatten(files, diskLen))
}
