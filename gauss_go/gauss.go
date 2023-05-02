package main

import ("fmt"
	"math/rand"
	"strconv"
	"os"
	"time"
)

const MAX = 10000
var N int
var A [MAX][MAX]float32
var B [MAX]float32
var X [MAX]float32



func parameters() {
	var args = os.Args
	var tam = len(args)

	if tam != 2 {
		panic("Uso: 'go run . <tamanho_matriz>'")
	}

	var err error

	N, err = strconv.Atoi(args[1])

	if (err != nil )|| (N > MAX - 1 || N < 1 ){
		fmt.Printf("Valor '%s' inválido ", args[1])
		panic("")
	}

	/* Print parameters */
	fmt.Println("\nMatrix dimension N =", N);
}

func random() (float32){
	var value = (float32(rand.Intn(1000)) * rand.Float32())

	return value
}
  
  /* Initialize A and B (and X to 0.0s) */
func initialize_inputs() {
	fmt.Println("Initializing...");
	for col := 0; col < N; col++ {
		for row := 0; row < N; row++ {
			A[row][col] = random()
		}
		B[col] = random();
		X[col] = 0.0;
	}

}

  /* Print input matrices */
func print_inputs() {
	if (N < 10) {
		fmt.Print("\nA =\n\t")
		for row := 0; row < N; row++ {
			for col := 0; col < N; col++ {
				if col != N - 1{
					fmt.Printf("%5.2f, ", A[row][col])
				} else {
					fmt.Printf("%5.2f,", A[row][col])
				}
			}
			fmt.Print("\n\t")
		}

		fmt.Printf("\nB = [ ")

		for col := 0; col < N; col++ {
			if col != N - 1{
				fmt.Printf("%5.2f, ", B[col])
			} else {
				fmt.Printf("%5.2f ]", B[col])
			}
		}
	}
}

func print_X() {
	if (N < 10) {
		fmt.Print("\nX = [ ");
		for row := 0; row < N; row++ {
			if row != N - 1{
				fmt.Printf("%5.2f, ", X[row])
			} else {
				fmt.Printf("%5.2f ]", X[row])
			}
		}
	}
}

func main()  {
	// var sum int
	
	parameters()

	initialize_inputs()

	print_inputs()

	start := time.Now()

	gauss()

	elapsed := time.Since(start)

	fmt.Println(elapsed)

	print_X()
}

func gauss(){

	var multiplier float32

	for norm := 0; norm < N - 1; norm++ {
		for row := norm + 1; row < N; row++ {
			multiplier = A[row][norm] / A[norm][norm];
			for col := norm; col < N; col++ {
				A[row][col] -= A[norm][col] * multiplier;
			}
			B[row] -= B[norm] * multiplier;
		}
	}
	/* (Diagonal elements are not normalized to 1.  This is treated in back
	 * substitution.)
	 */
  
  
	/* Back substitution */
	for row := N - 1; row >= 0; row-- {
		X[row] = B[row];
		for col := N-1; col > row; col-- {
			X[row] -= A[row][col] * X[col];
		}
		X[row] /= A[row][row];
	}

}

func Benchmarkgauss(){

	var multiplier float32

	for norm := 0; norm < N - 1; norm++ {
		for row := norm + 1; row < N; row++ {
			multiplier = A[row][norm] / A[norm][norm];
			for col := norm; col < N; col++ {
				A[row][col] -= A[norm][col] * multiplier;
			}
			B[row] -= B[norm] * multiplier;
		}
	}
	/* (Diagonal elements are not normalized to 1.  This is treated in back
	 * substitution.)
	 */
  
  
	/* Back substitution */
	for row := N - 1; row >= 0; row-- {
		X[row] = B[row];
		for col := N-1; col > row; col-- {
			X[row] -= A[row][col] * X[col];
		}
		X[row] /= A[row][row];
	}

}