import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;
import java.util.Arrays;
import java.util.ArrayList;
import java.util.List;

public class Puzzle1 {
    public static void main(String[] args) {
        try {
            List<Integer> expenseReport = new ArrayList<Integer>();
            File puzzle1File = new File("../puzzle1.txt");
            Scanner puzzle1Reader = new Scanner(puzzle1File);
            while (puzzle1Reader.hasNextLine()) {
                expenseReport.add(Integer.parseInt(puzzle1Reader.nextLine()));
            }
            Integer multiple;
            Integer target = 2020;
            multiple = sumExistsTwoNumbersMultiple(expenseReport, target);
            System.out.println(multiple);
            multiple = sumExistsThreeNumbersMultiple(expenseReport, target);
            System.out.println(multiple);
        } catch (FileNotFoundException exc) {
            System.out.println("An error occurred.");
            exc.printStackTrace();
        }
    }
    
    public static Integer sumExistsTwoNumbersMultiple(List<Integer> expenseReport, Integer target) {
        Integer delta;
        for (int i=0; i<expenseReport.size(); i++) {
            delta = target - expenseReport.get(i);
            if (expenseReport.contains(delta)) {
                return delta * expenseReport.get(i);
            }
        }
        return 0;
    }

    public static Integer sumExistsThreeNumbersMultiple(List<Integer> expenseReport, Integer target) {
        Integer firstNum;
        Integer secondNum;
        Integer delta;
        Integer secondDelta;
        List<Integer> deltaStore = new ArrayList<Integer>();
        for (int i=0; i<expenseReport.size(); i++) {
            firstNum = expenseReport.get(i);
            delta = target - firstNum;
            for (int j=i+1; j<expenseReport.size(); j++) {
                secondNum = expenseReport.get(j);
                secondDelta = delta - secondNum;
                if (expenseReport.subList(j, expenseReport.size()).contains(secondDelta)) {
                    return firstNum * secondNum * secondDelta;
                }
            }
        }
        return -1;
    }
}