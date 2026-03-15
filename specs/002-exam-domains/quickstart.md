# Quickstart: Exam Domains

The MLA-C01 Practice Tool now validates the `domain` field on all questions.

## Upgrading a Custom Question Bank

If you use `--bank my_questions.json`, ensure each question object defines `"domain": 1`, `2`, `3`, or `4` at the root object level. The application will panic/crash if the standard is not met.

```json
{
  "id": "my_bank",
  "version": "1.0",
  "questions": [
    {
      "id": "q-123",
      "domain": 1,
      "prompt": "How do you handle missing values?",
      "choices": ["Mean imputation", "Delete database", "Do nothing"],
      ...
    }
  ]
}
```

## Practice Mode: Single Domain Drilling

When starting Practice mode, the terminal will intercept you after selecting "Practice" to ask if you want "All Domains" or a specific "Domain 1-4". This limits the shuffled array presented to you to solely matching domain questions.
