<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Name_CO</name>
   <tag></tag>
   <elementGuidId>3b236938-4969-4c67-8e4a-e47b03df6562</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap.nameonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:nameNormalize>
         &lt;!--Optional:-->
         &lt;name>
            &lt;!--Optional:-->
            &lt;apellidos> &lt;/apellidos>
            &lt;!--Optional:-->
            &lt;genero> &lt;/genero>
            &lt;!--Optional:-->
            &lt;nivel1> &lt;/nivel1>
            &lt;!--Optional:-->
            &lt;nombres>Elvis Antonio Goncalves&lt;/nombres>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;sector> &lt;/sector>
            &lt;!--Optional:-->
            &lt;userName> &lt;/userName>
         &lt;/name>
      &lt;/soap:nameNormalize>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>nameNormalize</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Nombres</defaultValue>
      <description></description>
      <id>3472ae4f-d868-4224-b847-b44492c568c3</id>
      <masked>false</masked>
      <name>Nombres</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()







assertThat(response.getResponseText()).contains('&lt;apellidos>GONCALVES&lt;/apellidos>')
assertThat(response.getResponseText()).contains('&lt;estado>CO&lt;/estado>')
assertThat(response.getResponseText()).contains('&lt;genero>M&lt;/genero>')
assertThat(response.getResponseText()).contains('&lt;motivo>&lt;/motivo>')
assertThat(response.getResponseText()).contains('&lt;nivel1>&lt;/nivel1>')
assertThat(response.getResponseText()).contains('&lt;nombres>ELVIS ANTONIO&lt;/nombres>')
assertThat(response.getResponseText()).contains('&lt;tipoPersona>PF&lt;/tipoPersona>')</verificationScript>
   <wsdlAddress>${Nombres}</wsdlAddress>
</WebServiceRequestEntity>
