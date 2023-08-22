class Solution(object):
    def numUniqueEmails(self, emails: list[str]) -> int:
        unique_emails = set()
        for email in emails:
            user, domain = email.split('@')
            user = user.replace('.', '').split('+')[0]
            unique_emails.add(user + '@' + domain)

        return len(unique_emails)
