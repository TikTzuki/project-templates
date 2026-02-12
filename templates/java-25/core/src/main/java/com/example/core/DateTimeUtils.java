package com.example.core;

import java.time.Instant;
import java.time.ZoneOffset;
import java.time.ZonedDateTime;

public final class DateTimeUtils {

  private DateTimeUtils() {}

  public static Instant nowUtc() {
    return Instant.now();
  }

  public static ZonedDateTime nowZonedUtc() {
    return ZonedDateTime.now(ZoneOffset.UTC);
  }
}
