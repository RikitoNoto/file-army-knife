import Head from 'next/head'
import { open } from "@tauri-apps/api/dialog";
import styles from '/styles/line_counter.module.css'
import {initializeIcons} from '@fluentui/react'
import 'bootstrap/dist/css/bootstrap.min.css';
import {Button, Card, Col, Row} from "react-bootstrap"
import Header from 'components/layouts/header'
import SearchCard from 'components/elements/line_counter/search_card'
import React from "react";

initializeIcons();

type Props = {
}

class LineCounterPage  extends React.Component<Props, {}> {
	render(): React.ReactNode {
    return (
      <div className={styles.container}>
        <Head>
          <title>Line counter</title>
          {/* <meta name="description" content="Generated by create next app" /> */}
          <link rel="icon" href="/favicon.ico" />
        </Head>

        <main className={styles.main}>
          <Header></Header>

          <Card className={`${styles.path_container} col-11`}>
            <Card.Body>
              <Row>
                <Col xs={10}>
                  <Card body className={`${styles.path_display}`}>please select path.</Card>
                </Col>
                <Col xs={2} className={`${styles.select_button_container}`}>
                  <Button variant="outline-primary" type='button'>Select</Button>
                </Col>
              </Row>
            </Card.Body>
          </Card>
          <SearchCard gridSize={12}></SearchCard>

          <Button variant="primary" type='button' className={`${styles.add_button} rounded-circle`}>+</Button>
        </main>

        <footer className={styles.footer}>
        </footer>
      </div>
    );
  }


}

export default LineCounterPage;