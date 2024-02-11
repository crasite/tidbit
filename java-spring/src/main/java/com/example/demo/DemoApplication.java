package com.example.demo;

import java.util.List;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@SpringBootApplication
@RestController
public class DemoApplication {
  private int a = 0;

  public static void main(String[] args) {
    SpringApplication.run(DemoApplication.class, args);
  }

  @GetMapping("/hello")
  public String hello(@RequestParam(value = "name", defaultValue = "World") String name) {
    a += 1;
    return String.format("Hello %s! (%s)", name, a);
  }

  @GetMapping("/world")
  public List<String> world(@RequestParam(value = "name", defaultValue = "World") String name) {
    a += 1;
    return List.of(String.format("Hello %s! (%s)", name, a), "hi");
  }
}
