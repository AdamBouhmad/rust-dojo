use mail_send::*;
use mail_builder::*;

fn emailbuilder(matches_thisweek: u8, timespent_thisweek: u32, total_ranked_matches: u8, longest_game: u32) -> MessageBuilder<'static> {

    mail_builder::MessageBuilder::new()
        .from(("Stats", "stats@lolinsight.gg"))
        .to("you@you.com")
        .subject("The weekly LoLInsights Roundup")
        .text_body(format!("Total matches played this week: {} \n Total time played this week: {} \n Total ranked matches this week: {} \n Longest Game this week: {}", matches_thisweek, timespent_thisweek, total_ranked_matches, longest_game))

}

pub async fn emailservice(matches_thisweek: u8, timespent_thisweek: u32, total_ranked_matches: u8, longest_game: u32) {

     SmtpClientBuilder::new("mail.privateemail.com", 587)
        .implicit_tls(false)
        .credentials(("username", "supersecretpassword"))
        .connect()
        .await
        .unwrap()
        .send(emailbuilder(matches_thisweek, timespent_thisweek, total_ranked_matches, longest_game))
        .await
        .unwrap();
}