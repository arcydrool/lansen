<script setup lang="ts">
import { reactive } from 'vue';
import { zodResolver } from '@primevue/forms/resolvers/zod'
import { z } from 'zod';
import { Card, Button, InputText, InputMask, Textarea, Message, Checkbox, CheckboxGroup } from 'primevue';
import { Form, FormField, type FormSubmitEvent } from '@primevue/forms';

const state = reactive({ summary: "", severity: "" });

const initialValues = reactive({
  name: '',
  company: '',
  email: '',
  tel: '',
  interests: [],
  additional: ''
});

const resolver = zodResolver(
  z.object({
    name: z.string().min(1, "Please provide a name"),
    company: z.string().min(1, "Please provide a company"),
    email: z.string().email("Please provide an e-mail"),
    tel: z.string().optional(),
    mail: z.string().optional(),
    interests: z.array(z.string()).min(1, "Please Select At Least One Interest"),
    additional: z.string().optional()
  })
);
const onFormSubmit = (submitEvent: FormSubmitEvent): FormSubmitEvent => {
  if (submitEvent.valid) {
    const itemStr = localStorage.getItem('csub');
    let itemStr2: Object | null = null;
    if (itemStr) {
      const item = JSON.parse(itemStr);
      const now = new Date();
      const expiry = new Date(item.expiry);

      if (now.getTime() > expiry.getTime()) {
        localStorage.removeItem('csub');
      } else {
        itemStr2 = itemStr;
        state.summary = 'We ask that you use the contact form no more than once per day. [' + expiry.toLocaleDateString() + ' ' + expiry.toLocaleTimeString() + ']';
        state.severity = 'wait';
      }
    }
    if (!itemStr2) {
      const today = new Date();
      const tomorrow = new Date(today);
      tomorrow.setDate(today.getDate() + 1);

      const item = {
        value: 1,
        expiry: tomorrow,
      };
      try {
        const result = postContact(submitEvent);
        result
          .then((data) => {
            localStorage.setItem('csub', JSON.stringify(item));
            state.summary = 'Form is submitted. We will contact you shortly.';
            state.severity = 'success';
          })
          .catch((error) => {
            state.summary = 'Form submission failed. Please contact us a different way.';
            state.severity = 'fail';
          });

      } catch (error) {
        state.summary = 'Form submission failed. Please contact us a different way.';
        state.severity = 'fail';
      }
    }
  }
  return submitEvent
}

interface NoblePayload {
  name: string;
  company: string;
  email: string;
  tel: string;
  mail: string;
  interests: Array<String>;
  additional: string;
}

async function postJson(url: string, data: NoblePayload): Promise<Response> {
  const response = await fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(data)
  });

  if (!response.ok) {
    throw new Error(`Blast! Our letter was not received favorably: ${response.statusText}`);
  }
  return await response.json();
}

async function postContact(form: FormSubmitEvent): Promise<Response> {
  const payload: NoblePayload = { name: form.states.name?.value, email: form.states.email?.value, tel: form.states.tel?.value, company: form.states.company?.value, mail: form.states.mail?.value, interests: form.states.interests?.value, additional: form.states.additional?.value, };
  console.log(payload);
  return await postJson("/c", payload);
}
</script>
<template>
  <Card>
    <template #header>Contact Form:</template>
    <template #content>
      <Form v-slot="$form" :resolver="resolver" :initialValues="initialValues" @submit.default="onFormSubmit">
        <FormField as="section" name="name" initialValue="">
          <label for="name">Name</label>
          <InputText inputId="name" placeholder="Name" fluid />
          <Message v-if="$form.name?.invalid" severity="error" size="small" variant="simple">{{ $form.name.error.message
          }}</Message>
        </FormField>
        <FormField as="company" name="company" initialValue="">
          <label for="company">Company</label>
          <InputText inputId="company" placeholder="Company" fluid />
          <Message v-if="$form.company?.invalid" severity="error" size="small" variant="simple">{{
            $form.company.error.message }}</Message>
        </FormField>
        <FormField as="email" name="email" initialValue="">
          <label for="email">Email</label>
          <InputText inputId="email" placeholder="email@domain.com" fluid />
          <Message v-if="$form.email?.invalid" severity="error" size="small" variant="simple">{{
            $form.email.error.message
          }}</Message>
        </FormField>
        <FormField as="tel" name="tel" initialValue="">
          <label for="tel">Telephone</label>
          <InputMask inputId="tel" mask="999-999-9999" placeholder="413-555-5555" fluid />
          <Message v-if="$form.tel?.invalid" severity="error" size="small" variant="simple">{{ $form.tel.error.message
          }}
          </Message>
        </FormField>
        <FormField as="mail" name="mail" initialValue="">

          <label for="mail">Address</label><br>
          <Textarea inputId="mail" size="large" rows="5" placeholder="Business
1 Street Address
Berkshire, MA, 01224"></Textarea>
        </FormField>
        <div class="flex flex-col gap-2 border-2">
          <Message v-if="$form.interests?.invalid" severity="error" size="small" variant="simple">{{
            $form.interests.error.message }}</Message>
          <CheckboxGroup name="interests">
            <Checkbox inputId="custom" value="custom" />
            <label for="custom"> Custom Molds </label>
            <Checkbox inputId="inject" value="inject" />
            <label for="inject"> Injection Molding </label>
            <Checkbox inputId="specimen" value="specimen" />
            <label for="specimen"> Specimen Molding </label>
            <Checkbox inputId="plaque" value="plaque" />
            <label for="plaque"> Plaque Molding </label>
            <Checkbox inputId="other" value="other" />
            <label for="other"> Other </label>
          </CheckboxGroup>

        </div>
        <FormField as="additional" name="additional" initialValue="" class="flex felx-col gap-2">
          <label for="additional">Comments Or Questions</label><br>
          <Textarea inputId="additional" size="large" rows="5" placeholder="Additional Details"></Textarea>
          <Message v-if="$form.additional?.invalid" severity="error" size="small" variant="simple">{{
            $form.additional.error.message }}</Message>
        </FormField>
        <Message v-if="state.severity == 'wait'" severity="warn" size="small" variant="outlined">{{
          state.summary }}</Message>
        <Message v-if="state.severity == 'success'" severity="success" variant="outlined">{{
          state.summary }}</Message>
        <Message v-if="state.severity == 'fail'" severity="error" size="small" variant="outlined">{{
          state.summary }}</Message>
        <Button type="submit" severity="secondary" label="Submit" fluid></Button>
      </Form>
    </template>
  </Card>
</template>