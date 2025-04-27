package utils

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func GetKey[K comparable, V any](m map[K]V) []K {
	keys := make([]K, 0, len(m))
	for k := range m {
		keys = append(keys, k)
	}
	return keys
}
