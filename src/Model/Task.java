package Model;

import java.time.LocalDate;

public class Task {
    private Integer id;
    private String description;
    private Status status;
    private LocalDate createdAt;
    private LocalDate updatedAt;


    public enum Status {
        TODO, IN_PROGRESS, DONE
    }
}


